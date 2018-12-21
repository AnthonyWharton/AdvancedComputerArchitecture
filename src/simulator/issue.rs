use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// This is responsible for the _Issue_ stage of the pipeline, taking
/// pending instructions from the
/// [`ResvStation`](../reservation/struct.ResvStation.html) to free
/// [`ExecuteUnit`s](../execute/struct.ExecuteUnit.html).
pub fn issue_stage(state_p: &State, state: &mut State) {
    let mut effective_limit = state.issue_limit; 
    for eu in state.execute_units.iter_mut() {
        let (next, new_limit) = state_p
            .resv_station
            .consume_next(
                &mut state.resv_station,
                &eu,
                &state.reorder_buffer,
                effective_limit,
            );
        effective_limit = new_limit;
        if let Some(r) = next {
            eu.handle_issue(state_p, &r);
            if effective_limit == 0 {
                break;
            }
        }
    }
}
