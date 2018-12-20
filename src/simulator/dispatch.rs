use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// This is responsible for the _Dispatch_ stage of the pipeline, taking
/// pending instructions from the
/// [`ResvStation`](../reservation/struct.ResvStation.html) to free
/// [`ExecuteUnit`s](../execute/struct.ExecuteUnit.html).
pub fn dispatch_stage(state_p: &State, state: &mut State) {
    let mut effective_limit = state.dispatch_limit;
    for eu in state.execute_units.iter_mut() {
        let (next, new_limit) = state_p
            .resv_station
            .consume_next(
                &mut state.resv_station,
                &eu,
                &state_p.reorder_buffer,
                effective_limit,
            );
        effective_limit = new_limit;
        if let Some(r) = next {
            eu.handle_dispatch(state_p, &r);
            if effective_limit == 0 {
                break;
            }
        }
    }
}
