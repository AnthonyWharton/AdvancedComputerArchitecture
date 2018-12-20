use crate::isa::Instruction;
use crate::isa::op_code::Operation;
use crate::isa::operand::Register;
use crate::util::config::Config;

use super::memory::Access;
use super::register::RegisterFile;
use super::reorder::ReorderEntry;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BranchState {
    StronglyNotTaken,
    WeaklyNotTaken,
    WeaklyTaken,
    StronglyTaken,
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
    /// The return address stack.
    pub return_stack: Option<Vec<usize>>,
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
            return_stack: if config.return_address_stack {
                Some(vec![])
            } else {
                None
            },
            global_prediction: BranchState::default(),
        }
    }

    /// Predicts the next program counter for the _fetch_ stage to fetch to
    /// fetch the next instruction from.
    pub fn get_prediction(&self) -> usize {
        self.lc
    }

    /// The feedback from the _fetch_ stage as to last instruction that was
    /// loaded from memory, used to make the next prediction. Returns the next
    /// prediction to allow for easy implementation of the forward bypass.
    pub fn predict(&mut self, n_way: usize, next_instrs: &Vec<Access<i32>>, rf: &RegisterFile) {
        if self.enabled {
            for raw in next_instrs.iter() {
                let instr = match Instruction::decode(raw.word) {
                    Some(instr) => instr,
                    None => {
                        break
                    }
                };

                // If return stack optimisation is used and provides a
                // prediction, use it. Side effect of updating return stack.
                if let Some(pc) = self.process_return_address(instr, self.lc) {
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
                               // Requires updateing should_halt_decode()
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
                                   // Requires updateing should_halt_decode()
                        }
                    }
                    _ => self.lc += 4,
                }
            }
        } else {
            self.lc += 4 * n_way;
        }
    }

    /// Feedback on how the branch actually went from the _commit_ stage.
    pub fn commit_feedback(&mut self, rob_entry: &ReorderEntry) {
        if rob_entry.pc + 4 == rob_entry.act_pc as usize {
            self.global_prediction = BranchState::not_taken(self.global_prediction);
        } else {
            self.global_prediction = BranchState::taken(self.global_prediction);
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
    fn process_return_address(&mut self, instr: Instruction, pc: usize) -> Option<usize> {
        if let Some(stack) = &mut self.return_stack {
            match instr.op {
                Operation::JAL => {
                    if instr.rd == Some(Register::X1) || instr.rd == Some(Register::X5) {
                        stack.push(pc + 4)
                    }
                    None
                }
                Operation::JALR => {
                    let rd = instr.rd == Some(Register::X1) || instr.rd == Some(Register::X5);
                    let rs1 = instr.rs1 == Some(Register::X1) || instr.rs1 == Some(Register::X5);
                    let eq = instr.rd == instr.rs1;

                    if !rd && !rs1 {
                        None
                    } else if !rd && rs1 {
                        stack.pop()
                    } else if rd && !rs1 {
                        stack.push(pc + 4);
                        None
                    } else if rd && rs1 && !eq {
                        let ret = stack.pop();
                        stack.push(pc + 4);
                        ret
                    } else {
                        stack.push(pc + 4);
                        None
                    }
                }
                _ => None
            }
        } else {
            None
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
