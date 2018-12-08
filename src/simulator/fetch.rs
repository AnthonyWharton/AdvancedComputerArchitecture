use super::memory::Access;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The contents of the latch that the fetch stage feeds into.
#[derive(Clone, Default)]
pub struct LatchFetch {
    /// The data fetched.
    pub data: Access<i32>,
    /// The program counter value for this instruction, indicating the choice
    /// the branch predictor made.
    pub pc: usize,
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The fetch stage of the pipeline. This will fetch the next instruction(s)
/// from memory, and put them into a latch ready for the next pipeline stage.
///
/// Requires previous self to be mutable due to mutable requirement on
/// [`Memory.read_i32()`](../memory/struct.Memory.html#method.write_i32).
/// Nothing else in the state will be changed.
pub fn fetch_stage(state_p: &State, state_n: &mut State) {
    // Get branch prediction fed in
    let pc = state_p.branch_predictor.get_prediction();
    // Load word
    let data = state_p.memory.read_i32(pc);
    // Pass loaded word to following latch and branch predictor.
    state_n.branch_predictor.predict(data.word);
    state_n.latch_fetch = LatchFetch {
        data,
        pc,
    };
}

