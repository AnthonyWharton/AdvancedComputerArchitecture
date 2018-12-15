use either::Left;

use crate::isa::Instruction;

use super::reorder::ReorderEntry;
use super::reservation::Reservation;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The _Decode & Rename_ stage of the pipeline. This will decode
/// instruction(s) from the previous
/// [`LatchFetch`](../fetch/struct.LatchFetch.html) in the pipeline; perform
/// register renaming; allocate a slot in the
/// [`ReorderBuffer`](../reorder/struct.ReorderBuffer.html) and then place into
/// the next stage in the pipeline, the
/// [`ResvStation`](../reservation/struct.ResvStation.html).
///
/// If sanitisation is not possible, this will stall the pipeline.
pub fn decode_and_rename_stage(state_p: &State, state: &mut State) {
    if state_p.finish_rob_entry.is_some() {
        return
    }

    if let Some(access) = state_p.latch_fetch.data {
        let instr = match Instruction::decode(access.word) {
            Some(i) => i,
            None => panic!("Failed to decode instruction."),
        };

        let resv_result = sanitise_and_reserve(
            instr,
            state_p.latch_fetch.pc,
            state,
        );

        if resv_result.is_err() {
            state.branch_predictor.stall();
            state.stats.stalls += 1;
        }
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
    pc: usize,
    state: &mut State,
) -> Result<(), ()> {
    // Check RS and ROB both have free capacity for a reservation
    if !state.resv_station.free_capacity() || !state.reorder_buffer.free_capacity() {
        return Err(());
    }

    // Reserve a physical register for writeback.
    let mut name_rd = 0;
    if let Some(rd) = instruction.rd {
        match state.register.using_write(rd) {
            Some(n) => name_rd = n,
            None => return Err(()), // No Available Physical Registers
        }
    }

    // Get renamed registers for instruction (if required)
    let rs1 = match instruction.rs1 {
        Some(rs1) => state.register.using_read(rs1),
        None => Left(0),
    };
    let rs2 = match instruction.rs2 {
        Some(rs2) => state.register.using_read(rs2),
        None => Left(0),
    };

    // Reserve a reorder buffer entry
    let reorder_entry = ReorderEntry {
        finished: false,
        op: instruction.op,
        pc,
        act_pc: 0,
        act_rd: 0,
        reg_rd: instruction.rd,
        name_rd: match instruction.rd {
            Some(_) => Some(name_rd),
            None => None,
        },
        rs1,
        rs2,
        imm: instruction.imm,
    };
    let rob_entry = match state.reorder_buffer.reserve_entry(reorder_entry) {
        Some(entry) => entry,
        None => panic!("ROB was free at start of reservation stage but not at the end!"),
    };

    // Check if the instruction was a finish instruction
    if instruction.is_ret() {
        state.finish_rob_entry = Some(rob_entry);
    }

    // Finally, reserve the instruction in the reservation station
    let reservation = Reservation {
        rob_entry,
        pc,
        op: instruction.op,
        reg_rd: instruction.rd,
        name_rd: match instruction.rd {
            Some(_) => Some(name_rd),
            None => None,
        },
        rs1,
        rs2,
        imm: instruction.imm,
    };
    match state.resv_station.reserve(reservation) {
        Ok(()) => Ok(()),
        Err(()) => panic!("RS was free at start of reservation stage but not at the end!"),
    }
}
