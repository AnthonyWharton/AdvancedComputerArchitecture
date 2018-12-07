use super::execute::ExecuteUnit;
use super::state::State;
use super::register::RegisterFile;
use super::reservation::ResvStation;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

pub fn dispatch(state_p: &State, state_n: &mut State) {
    unimplemented!();
    let mut effective_limit = state_p.execute_units.len();
    for eu in exec_units.iter() {
        if eu.is_free() {
            let next = rs.consume_next(
                eu.get_type(),
                &state_p.register,
                effective_limit
            );
            if let Some(r) = next {
                eu.handle_execute(state_p, r);
                effective_limit -= 1;
                if effective_limit == 0 {
                    break
                }
            }
        }
    }
}

