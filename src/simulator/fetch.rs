use super::branch::ReturnStackOp;
use super::memory::Access;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The contents of the latch that the fetch stage feeds into.
#[derive(Clone, Debug, Default)]
pub struct LatchFetch {
    /// The `n` pieces of data fetched (determined by the
    /// [State's](../state/struct.State.html) `n_way` settings).
    pub data: Vec<Access<i32>>,
    /// The `n` pieces of associate branch prediction data for the fetched
    /// memory address. This include the return stack operations as well as
    /// the two level prediction state.
    pub bp_data: Vec<(ReturnStackOp, u8)>,
    /// The program counter value for the _first_ instruction fetched,
    /// indicating the choice the branch predictor made.
    pub pc: usize,
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The _Fetch_ stage of the pipeline. This will fetch the next instruction(s)
/// from [`Memory`](../memory/struct.Memory.html), and put them into the
/// [`LatchFetch`](../fetch/struct.LatchFetch.html) ready for the next pipeline
/// stage.
pub fn fetch_stage(state_p: &State, state: &mut State) {
    let lc = state_p.branch_predictor.get_prediction();
    let mut data = vec![];
    for offset in 0..state_p.n_way {
        data.push(state_p.memory.read_i32(lc + (4 * offset)))
    }
    let bp_data = state.branch_predictor.predict(state_p.n_way, &data, &state_p.register);
    state.latch_fetch = LatchFetch { data, bp_data, pc: lc };
}
