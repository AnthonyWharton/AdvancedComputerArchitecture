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
        }
    }

    /// Predicts the next program counter for the _fetch_ stage to fetch to
    /// fetch the next instruction from.
    pub fn get_prediction(&self) -> usize {
        self.lc
    }

    /// The feedback from the fetch stage as to last instruction that was
    /// loaded from memory, used to make the next prediction. Returns the next
    /// prediction to allow for easy implementation of the forward bypass.
    pub fn predict(&mut self, _next_instr: i32) {
        self.old_lc = self.lc;
        self.lc += 4;
    }

    /// Feedback from the decode unit that a stall has occured due to resources
    /// not being available. This should roll back to the previous predictive
    /// state.
    pub fn stall(&mut self) {
        self.lc = self.old_lc;
    }

    /// Feedback from the execution unit that the earlier prediction was
    /// incorrect, and that the program counter should be hard reset to the
    /// given value.
    pub fn force_update(&mut self, corrected_pc: usize) {
        self.lc = corrected_pc;
    }
}
