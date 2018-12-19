use either::{Either, Left, Right};

use crate::isa::Instruction;
use crate::isa::operand::Register;

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
    if let Some(access) = state_p.latch_fetch.data {
        let instr = match Instruction::decode(access.word) {
            Some(i) => i,
            None => {
                state.branch_predictor.stall();
                state.stats.stalls += 1;
                return
            },
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

    // Get renamed registers for instruction (if required)
    let rs1 = match instruction.rs1 {
        Some(rs1) => get_read(state, rs1),
        None => Left(0),
    };
    let rs2 = match instruction.rs2 {
        Some(rs2) => get_read(state, rs2),
        None => Left(0),
    };

    // Reserve a reorder buffer entry
    let reorder_entry = ReorderEntry {
        finished: false,
        ref_count: 0,
        op: instruction.op,
        pc,
        act_pc: 0,
        act_rd: None,
        reg_rd: instruction.rd,
        rs1,
        rs2,
        imm: instruction.imm,
    };
    let rob_entry = match state.reorder_buffer.reserve_entry(reorder_entry) {
        Some(entry) => entry,
        None => panic!("ROB was free at start of reservation stage but not at the end!"),
    };

    // Rename register in register file
    if let Some(reg) = instruction.rd {
        state.register.rename(reg, rob_entry);
    }

    // Finally, reserve the instruction in the reservation station
    let reservation = Reservation {
        rob_entry,
        pc,
        op: instruction.op,
        reg_rd: instruction.rd,
        rs1,
        rs2,
        imm: instruction.imm,
    };
    match state.resv_station.reserve(reservation) {
        Ok(()) => Ok(()),
        Err(()) => panic!("RS was free at start of reservation stage but not at the end!"),
    }
}

/// Either returns the valid value of the given register, or the reorder buffer
/// entry that will hold the required result when ready.
fn get_read(state: &mut State, register: Register) -> Either<i32, usize> {
    if state.register[register].rename.is_none() {
        Left(state.register[register].data)
    } else {
        let rename = state.register[register].rename.unwrap();
        state.reorder_buffer[rename].ref_count += 1;
        Right(rename)
    }
}
