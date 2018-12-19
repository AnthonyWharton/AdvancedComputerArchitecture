use crate::isa::op_code::Operation;

use super::memory::Access;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The Branch Predictor's state and logic, responsible for informing the
/// _fetch_ stage of which address to read the next instruction from, in the
/// most informed way possible so as to have successful speculative execution.
#[derive(Clone)]
pub struct BranchPredictor {
    /// The internal load counter as kept track of by the branch predictor.
    pub lc: usize,
    /// The previous load counter as kept track of by the branch predictor,
    /// used to roll back one step in the event of a stall signal.
    pub old_lc: usize,
    /// Whether or not non-trivial branch prediction is enabled.
    pub enabled: bool
    // TODO, add relevant state for more complex Branch Prediction.
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl BranchPredictor {
    /// Creates a new Branch Predictor with an initial program counter, which
    /// will be the first address to be loaded.
    pub fn new(inital_pc: usize) -> BranchPredictor {
        BranchPredictor {
            lc: inital_pc,
            old_lc: inital_pc,
            enabled: false,
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
    pub fn predict(&mut self, n_way: usize, _next_instrs: &Vec<Access<i32>>) {
        self.old_lc = self.lc;
        self.lc += 4 * n_way;
    }


    /// Feedback that the branch predictor should provide load counter from the
    /// given `corrected_pc` in the next cycle. This could be from a pipeline
    /// stall, or a pipeline flush from a mispredicted branch.
    pub fn force_update(&mut self, corrected_pc: usize) {
        self.lc = corrected_pc;
    }

    /// Whether or not the _decode_ state should halt allocating future
    /// instructions given that it has just decoded the given type of
    /// operation.
    pub fn should_halt_decode(&self, _operation: Operation) -> bool {
        if !self.enabled {
            return false;
        }
        unimplemented!()
    }
}
