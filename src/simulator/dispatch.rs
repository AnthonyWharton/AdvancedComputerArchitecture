use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

pub fn dispatch(state_p: &State, state: &mut State) {
    let mut effective_limit = state_p.execute_units.len();
    for eu in state.execute_units.iter_mut() {
        let next = state.resv_station.consume_next(
            &eu,
            &state_p.register,
            effective_limit
        );
        if let Some(r) = next {
            eu.handle_execute(state_p, &r);
            effective_limit -= 1;
            if effective_limit == 0 {
                break
            }
        }
    }
}

