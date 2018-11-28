use isa::operand::Register;

use super::memory::Access;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The contents of the latch that the fetch stage feeds into.
#[derive(Clone, Default)]
pub struct LatchFetch {
    /// The data fetched.
    data: Option<Access<i32>>,
    /// The choice the branch predictor made.
    branch_choice: u32,
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The fetch stage of the pipeline. This will fetch the next instruction(s)
/// from memory, and put them into a latch ready for the next pipeline stage.
pub fn fetch_stage(state_p: &mut State, state_n: &mut State) {
    if state_p.l_fetch.is_none() && state_p.l_decode.is_none() {
        state_n.l_fetch = Some(state_p.memory.read_i32(
            state_p.register[Register::PC as usize] as usize
        ));
    }
}

