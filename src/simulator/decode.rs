use either::Left;

use isa::Instruction;
use super::state::State;
use super::register::RegisterFile;
use super::reorder::ReorderBuffer;
use super::reservation::{Reservation, ResvStation};

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

/// Handles all the reservations for a decoded instruction.
///
///   1) Renames the writeback register.
///   2) Reserves a slot in the reorder buffer.
///   3) Creates a reservation in the reservation station.
///
/// Should always undo any resource allocations should a resource not be
/// available when being run in a single thread.
///
/// Returns whether or not all reservations were made succesffully.
fn sanitise_and_reserve(
        instruction: Instruction,
        spec_bp_pc: usize,
        rob: &mut ReorderBuffer,
        rs: &mut ResvStation,
        rf: &mut RegisterFile,
) -> Result<(),()> {
    // Reserve a physical register for writeback.
    let mut name_rd = 0;
    match instruction.rd {
        Some(rd) => match rf.using_write(rd) {
            Some(n) => name_rd = n,
            None => return Err(()), // No Available Physical Registers
        },
        None => (), // No need to rename as no writeback.
    }

    // Reserve a reorder buffer entry
    let rob_entry = match rob.reserve_entry(spec_bp_pc) {
        Some(entry) => entry,
        None => {
            rf.not_using_write(name_rd);
            return Err(())
        },
    };

    // Check reservation station has availability and if so reserve an
    // instruction.
    if !rs.free_capactiy() {
        return Err(())
    }

    let reservation = Reservation {
        rob_entry,
        spec_bp_pc,
        op: instruction.op,
        reg_rd: instruction.rd,
        name_rd: match instruction.rd {
                Some(_) => Some(name_rd),
                None => None,
            },
        rs1: match instruction.rs1 {
                Some(rs1) => rf.using_read(rs1),
                None => Left(0),
            },
        rs2: match instruction.rs2 {
                Some(rs2) => rf.using_read(rs2),
                None => Left(0),
            },
        imm: instruction.imm,
    };

    rs.reserve(reservation)
}
