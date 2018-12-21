use crate::isa::Instruction;
use crate::isa::op_code::Operation;
use crate::isa::operand::Register;
use crate::util::config::Config;

use super::memory::Access;
use super::register::RegisterFile;
use super::reorder::ReorderEntry;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

/// Number of levels to use for two level adaptive prediction.
/// *MUST* be a power of two.
const TWO_LEVEL: u8 = 1 << 3;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// The mode of operation that the branch predictor is in.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BranchPredictorMode {
    /// No branch prediction is enabled.
    Off,
    /// One bit saturating counter prediction enabled.
    OneBit,
    /// Two bit saturating counter prediction enabled.
    TwoBit,
    /// Two Level adaptive 3 bit predictor enabled.
    TwoLevel,
}

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
    pub mode: BranchPredictorMode,
    /// The dirty return address stack.
    pub return_stack_d: Option<Vec<usize>>,
    /// The clean return address stack.
    pub return_stack_c: Option<Vec<usize>>,
    /// The global saturating counter finite state machine for branch
    /// prediction choices.
    pub saturating_counter: BranchState,
    /// The branch states for the two level prediction.
    pub two_level_counter: Vec<BranchState>,
    /// The branch history for the two level prediction.
    pub two_level_history: u8,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl BranchPredictor {
    /// Creates a new Branch Predictor with an initial program counter, which
    /// will be the first address to be loaded.
    pub fn new(config: &Config) -> BranchPredictor {
        BranchPredictor {
            lc: 0,
            mode: config.branch_prediction,
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
            saturating_counter: BranchState::default(),
            two_level_counter: vec![BranchState::default(); TWO_LEVEL as usize],
            two_level_history: 0b0000,
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
    ) -> Vec<(ReturnStackOp, u8)>{
        if self.mode != BranchPredictorMode::Off {
            let mut bp_data = vec![];
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
               bp_data.push((rs_op, self.two_level_history));
                if let Some(pc) = rs_pred {
                    self.lc = pc;
                    break;
                }

                // Otherwise, stick with usual branch prediction method
                let (brk, pc) = self.process_saturating_counter(instr, rf);
                self.lc = pc;
                if brk {
                    break
                }
            }
            bp_data.resize(n_way, (ReturnStackOp::None, 0));
            bp_data
        } else {
            self.lc += 4 * n_way;
            vec![(ReturnStackOp::None, 0); 4]
        }
    }

    /// Feedback on how the branch actually went from the _commit_ stage, where
    /// `mismatch` is set when the branch prediction failed.
    pub fn commit_feedback(&mut self, rob_entry: &ReorderEntry, mismatch: bool) {
        if rob_entry.pc + 4 == rob_entry.act_pc as usize {
            // Sort out saturating counter
            self.saturating_counter = BranchState::not_taken(
                self.saturating_counter,
                self.mode == BranchPredictorMode::OneBit
            );

            // Sort out two level prediction
            self.two_level_counter[rob_entry.bp_data.1 as usize] = BranchState::not_taken(
                self.two_level_counter[rob_entry.bp_data.1 as usize],
                false
            );
            self.two_level_history = (self.two_level_history << 1) & (TWO_LEVEL - 1);
        } else {
            // Sort out saturating counter
            self.saturating_counter = BranchState::taken(
                self.saturating_counter,
                self.mode == BranchPredictorMode::OneBit
            );

            // Sort out two level prediction
            self.two_level_counter[rob_entry.bp_data.1 as usize] = BranchState::taken(
                self.two_level_counter[rob_entry.bp_data.1 as usize],
                false
            );
            self.two_level_history = ((self.two_level_history << 1) & (TWO_LEVEL - 1)) | 0b1;
        }

        // Sort out return stack
        self.apply_stack_operation(rob_entry.bp_data.0);
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
            Operation::BGEU => return self.mode != BranchPredictorMode::Off,
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

    fn process_saturating_counter(
        &mut self,
        instr: Instruction,
        rf: &RegisterFile,
    ) -> (bool, usize) {
        match instr.op {
            Operation::JALR => {
                let are = &rf[instr.rs1.unwrap()];
                if are.rename.is_none() {
                    let new_lc = (are.data + instr.imm.unwrap()) & !0b1;
                    // Don't jump to zero/minus 1 (end of execution)
                    if 0 < new_lc {
                        return (true, new_lc as usize)
                    }
                }
                (true, self.lc + 4)
            }
            Operation::JAL  |
            Operation::BEQ  |
            Operation::BNE  |
            Operation::BLT  |
            Operation::BGE  |
            Operation::BLTU |
            Operation::BGEU => {
                if self.saturating_counter.should_take() {
                    (true, ((self.lc as i32) + instr.imm.unwrap()) as usize)
                } else {
                    (false, self.lc + 4)
                }
            }
            _ => (false, self.lc + 4),
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

impl Default for BranchPredictorMode {
    /// Defaults to two bit saturating counter.
    fn default() -> BranchPredictorMode {
        BranchPredictorMode::TwoBit
    }
}

impl Default for BranchState {
    /// Defaults to weakly taken.
    fn default() -> BranchState {
        BranchState::WeaklyTaken
    }
}

impl BranchState {
    /// Return whether or not this state means that the branch should be taken
    pub fn should_take(&self) -> bool {
        *self == BranchState::StronglyTaken || *self == BranchState::WeaklyTaken
    }

    /// Moves the Branch State if the branch was taken. The `one_bit` flag
    /// signals that this should be a one bit saturating counter operation.
    pub fn taken(state: BranchState, one_bit: bool) -> BranchState {
        if state == BranchState::StronglyNotTaken {
            BranchState::WeaklyNotTaken
        } else if state == BranchState::WeaklyNotTaken {
            BranchState::WeaklyTaken
        } else if one_bit { // WeaklyTaken from here down
            BranchState::WeaklyTaken
        } else {
            BranchState::StronglyTaken
        }
    }

    /// Moves the Branch State if the branch was not taken. The `one_bit` flag
    /// signals that this should be a one bit saturating counter operation.
    pub fn not_taken(state: BranchState, one_bit: bool) -> BranchState {
        if state == BranchState::StronglyTaken {
            BranchState::WeaklyTaken
        } else if state == BranchState::WeaklyTaken {
            BranchState::WeaklyNotTaken
        } else if one_bit { // WeaklyNotTaken from here down
            BranchState::WeaklyNotTaken
        } else {
            BranchState::StronglyNotTaken
        }
    }
}
