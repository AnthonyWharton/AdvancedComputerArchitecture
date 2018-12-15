use either::Left;

use crate::isa::Instruction;

use super::register::RegisterFile;
use super::reorder::{ReorderBuffer, ReorderEntry};
use super::reservation::{Reservation, ResvStation};
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The decode and rename stage of the pipeline. This will decode
/// instruction(s) from the previous stage in the pipeline; check for
/// inter-instruction dependencies; sanitise any dependencies; and then place
/// into the next stage in the pipeline.
///
/// If sanitisation is not possible, this will stall the pipeline.
pub fn decode_and_rename_stage(state_p: &State, state: &mut State) {
    if let Some(access) = state_p.latch_fetch.data {
        let instr = match Instruction::decode(access.word) {
            Some(i) => i,
            None => panic!("Failed to decode instruction."),
        };

        let resv_result = sanitise_and_reserve(
            instr,
            state_p.latch_fetch.pc,
            &mut state.reorder_buffer,
            &mut state.resv_station,
            &mut state.register,
        );

        if resv_result.is_err() {
            state.branch_predictor.stall()
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
    rob: &mut ReorderBuffer,
    rs: &mut ResvStation,
    rf: &mut RegisterFile,
) -> Result<(), ()> {
    // Check RS and ROB both have free capacity for a reservation
    if !rs.free_capacity() || !rob.free_capacity() {
        return Err(());
    }

    // Reserve a physical register for writeback.
    let mut name_rd = 0;
    if let Some(rd) = instruction.rd {
        match rf.using_write(rd) {
            Some(n) => name_rd = n,
            None => return Err(()), // No Available Physical Registers
        }
    }

    // Get renamed registers for instruction (if required)
    let rs1 = match instruction.rs1 {
        Some(rs1) => rf.using_read(rs1),
        None => Left(0),
    };
    let rs2 = match instruction.rs2 {
        Some(rs2) => rf.using_read(rs2),
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
    let rob_entry = match rob.reserve_entry(reorder_entry) {
        Some(entry) => entry,
        None => panic!("ROB was free at start of reservation stage but not at the end!"),
    };

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
    match rs.reserve(reservation) {
        Ok(()) => Ok(()),
        Err(()) => panic!("RS was free at start of reservation stage but not at the end!"),
    }
}
