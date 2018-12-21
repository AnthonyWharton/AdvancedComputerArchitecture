use crate::isa::Instruction;
use crate::isa::op_code::Operation;
use crate::isa::operand::Register;
use crate::util::config::Config;

use super::memory::Access;
use super::register::RegisterFile;
use super::reorder::ReorderEntry;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// The branch prediction FSM state.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BranchState {
    StronglyNotTaken,
    WeaklyNotTaken,
    WeaklyTaken,
    StronglyTaken,
}

/// An operation to the return stack.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReturnStackOp {
    None,
    Pushed(usize),
    Popped,
    PushPop(usize),
}

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The Branch Predictor's state and logic, responsible for informing the
/// _fetch_ stage of which address to read the next instruction from, in the
/// most informed way possible so as to have successful speculative execution.
#[derive(Clone, Default)]
pub struct BranchPredictor {
    /// The internal load counter as kept track of by the branch predictor.
    pub lc: usize,
    /// Whether or not non-trivial branch prediction is enabled.
    pub enabled: bool,
    /// The dirty return address stack.
    pub return_stack_d: Option<Vec<usize>>,
    /// The clean return address stack.
    pub return_stack_c: Option<Vec<usize>>,
    /// The global finite state machine for branch prediction choices.
    pub global_prediction: BranchState,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl BranchPredictor {
    /// Creates a new Branch Predictor with an initial program counter, which
    /// will be the first address to be loaded.
    pub fn new(config: &Config) -> BranchPredictor {
        BranchPredictor {
            lc: 0,
            enabled: config.branch_prediction,
            return_stack_d: if config.return_address_stack {
                Some(vec![])
            } else {
                None
            },
            return_stack_c: if config.return_address_stack {
                Some(vec![])
            } else {
                None
            },
            global_prediction: BranchState::default(),
        }
    }

    /// Predicts the next program counter for the _fetch_ stage to fetch to
    /// fetch the next instruction from, along with a guarenteed `n_way` number
    /// of return stack operations to go with those instructions.
    pub fn get_prediction(&self) -> usize {
        self.lc
    }

    /// The feedback from the _fetch_ stage as to last instructions that were
    /// loaded from memory, used to make the next prediction. Returns the
    /// return address stack operations for the instructions that were fetched.
    pub fn predict(
        &mut self,
        n_way: usize,
        next_instrs: &Vec<Access<i32>>,
        rf: &RegisterFile,
    ) -> Vec<ReturnStackOp>{
        if self.enabled {
            let mut rs_ops = vec![];
            for raw in next_instrs.iter() {
                let instr = match Instruction::decode(raw.word) {
                    Some(instr) => instr,
                    None => {
                        break
                    }
                };

                // If return stack optimisation is used and provides a
                // prediction, use it.
                let (rs_op, rs_pred) = self.process_return_address(instr, self.lc);
                rs_ops.push(rs_op);
                if let Some(pc) = rs_pred {
                    self.lc = pc;
                    break;
                }

                // Otherwise, stick with usual branch prediction method
                match instr.op {
                    Operation::JALR => {
                        let are = &rf[instr.rs1.unwrap()];
                        if are.rename.is_none() {
                            let new_lc = (are.data + instr.imm.unwrap()) & !0b1;
                            // Don't jump to zero/minus 1 (end of execution)
                            if 0 < new_lc {
                                self.lc = new_lc as usize;
                                break;
                            }
                        }
                        self.lc += 4;
                        break; // TODO consider removing this?
                               // Requires updating should_halt_decode()
                    }
                    Operation::JAL  |
                    Operation::BEQ  |
                    Operation::BNE  |
                    Operation::BLT  |
                    Operation::BGE  |
                    Operation::BLTU |
                    Operation::BGEU => {
                        if self.global_prediction.should_take() {
                            self.lc = ((self.lc as i32) + instr.imm.unwrap()) as usize;
                            break;
                        } else {
                            self.lc += 4;
                            break; // TODO consider removing this?
                                   // Requires updating should_halt_decode()
                        }
                    }
                    _ => self.lc += 4,
                }
            }
            rs_ops.resize(n_way, ReturnStackOp::None);
            rs_ops
        } else {
            self.lc += 4 * n_way;
            vec![ReturnStackOp::None; 4]
        }
    }

    /// Feedback on how the branch actually went from the _commit_ stage, where
    /// `mismatch` is set when the branch prediction failed.
    pub fn commit_feedback(&mut self, rob_entry: &ReorderEntry, mismatch: bool) {
        // Sort out global FSM prediction
        if rob_entry.pc + 4 == rob_entry.act_pc as usize {
            self.global_prediction = BranchState::not_taken(self.global_prediction);
        } else {
            self.global_prediction = BranchState::taken(self.global_prediction);
        }

        // Sort out return stack
        self.apply_stack_operation(rob_entry.rs_operation);
        if mismatch {
            self.return_stack_d = self.return_stack_c.clone();
        }
    }

    /// Feedback that the branch predictor should reset to the load counter in
    /// the given `corrected_pc` in the next cycle. This could be from a
    /// pipeline stall, or a pipeline flush from a mispredicted branch.
    pub fn force_update(&mut self, corrected_pc: usize) {
        self.lc = corrected_pc;
    }

    /// Whether or not the _decode_ state should halt allocating future
    /// instructions given that it has just decoded the given type of
    /// operation.
    pub fn should_halt_decode(&self, operation: Operation) -> bool {
        match operation {
            Operation::JAL  |
            Operation::JALR |
            Operation::BEQ  |
            Operation::BNE  |
            Operation::BLT  |
            Operation::BGE  |
            Operation::BLTU |
            Operation::BGEU => return self.enabled,
            _ => return false,
        }
    }

    /// Process an instruction for the return address stack optimisation.
    /// Returns a popped return address program counter prediction if one is
    /// available.
    fn process_return_address(
        &mut self,
        instr: Instruction,
        pc: usize,
    ) -> (ReturnStackOp, Option<usize>) {
        if let Some(stack) = &mut self.return_stack_d {
            match instr.op {
                Operation::JAL => {
                    if let Some(rd) = instr.rd {
                        if rd == Register::X1 || rd == Register::X5 {
                            stack.push(pc + 4);
                            return (ReturnStackOp::Pushed(pc + 4), None)
                        }
                    }
                    (ReturnStackOp::None, None)
                }
                Operation::JALR => {
                    let rd = instr.rd == Some(Register::X1) || instr.rd == Some(Register::X5);
                    let rs1 = instr.rs1 == Some(Register::X1) || instr.rs1 == Some(Register::X5);
                    let eq = instr.rd == instr.rs1;

                    if !rd && !rs1 {
                        (ReturnStackOp::None, None)
                    } else if !rd && rs1 {
                        (ReturnStackOp::Popped, stack.pop())
                    } else if rd && !rs1 {
                        stack.push(pc + 4);
                        (ReturnStackOp::Pushed(pc + 4), None)
                    } else if rd && rs1 && !eq {
                        let ret = stack.pop();
                        stack.push(pc + 4);
                        (ReturnStackOp::PushPop(pc + 4), ret)
                    } else {
                        stack.push(pc + 4);
                        (ReturnStackOp::Pushed(pc + 4), None)
                    }
                }
                _ => (ReturnStackOp::None, None)
            }
        } else {
            (ReturnStackOp::None, None)
        }
    }

    /// Applies a `ReturnStackOp` to the return stack in the branch predictor,
    /// this will apply to the clean return stack.
    fn apply_stack_operation(&mut self, op: ReturnStackOp) {
        if let Some(stack) = &mut self.return_stack_c {
            match op {
                ReturnStackOp::None => (),
                ReturnStackOp::Popped => { stack.pop(); },
                ReturnStackOp::Pushed(pc) => stack.push(pc),
                ReturnStackOp::PushPop(pc) => {
                    stack.pop();
                    stack.push(pc)
                }
            }
        }
    }
}

impl Default for BranchState {
    fn default() -> BranchState {
        BranchState::WeaklyTaken
    }
}

impl BranchState {
    /// Return whether or not this state means that the branch should be taken
    pub fn should_take(&self) -> bool {
        *self == BranchState::StronglyTaken || *self == BranchState::WeaklyTaken
    }

    /// Moves the Branch State if the branch was taken.
    pub fn taken(state: BranchState) -> BranchState {
        if state == BranchState::StronglyNotTaken {
            BranchState::WeaklyNotTaken
        } else if state == BranchState::WeaklyNotTaken {
            BranchState::WeaklyTaken
        } else {
            BranchState::StronglyTaken
        }
    }

    /// Moves the Branch State if the branch was not taken.
    pub fn not_taken(state: BranchState) -> BranchState {
        if state == BranchState::StronglyTaken {
            BranchState::WeaklyTaken
        } else if state == BranchState::WeaklyTaken {
            BranchState::WeaklyNotTaken
        } else {
            BranchState::StronglyNotTaken
        }
    }
}
