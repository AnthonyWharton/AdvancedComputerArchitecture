use isa::Instruction;

use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The decode and rename stage of the pipeline. This will decode
/// instruction(s) from the previous stage in the pipeline; check for
/// inter-instruction dependencies; sanitise any dependencies; and then place
/// into the next stage in the pipeline.
///
/// If sanitisation is not possible, this will stall the pipeline.
pub fn decode_and_rename_stage(state_p: &State, state_n: &mut State) {
    if let Some(ref raw) = state_p.l_fetch {
        state_n.l_decode = match Instruction::decode(raw.word) {
            Some(i) => Some(i),
            None => { panic!("Failed to decode instruction.") },
        };
        state_n.l_fetch = None;
    }

}

