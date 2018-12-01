use std::collections::VecDeque;

use either::{Either, Left};

use isa::Instruction;
use isa::op_code::Operation;
use isa::operand::Register;
use super::register::RegisterFile;
use super::reorder::ReorderBuffer;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Reservation station data and logic.
#[derive(Clone)]
pub struct ResvStation {
    /// The amount of reservations the Reservation Station can hold.
    capacity: usize,
    /// The contents of the Reservation Station.
    contents: VecDeque<Reservation>,
}

/// A single Reservation within the Reservation Station.
#[derive(Clone)]
pub struct Reservation {
    /// The entry in the reorder buffer that corresponds to this entry.
    pub rob_entry: usize,
    /// The program counter value for this instruction, indicating the choice
    /// the branch predictor made.
    pub spec_bp_pc: usize,
    /// The pending operation
    pub op:  Operation,
    /// The pending writeback register.
    pub reg_rd: Option<Register>,
    /// The pending writeback register name.
    pub name_rd: Option<usize>,
    /// Either the first source register name, or value. If this argument is
    /// unused, it will be set as 0.
    pub rs1: Either<i32, usize>,
    /// Either the second source register name, or value. If this argument is
    /// unused, it will be set as 0.
    pub rs2: Either<i32, usize>,
    /// The immediate of the pending instruction, if applicable.
    pub imm: Option<i32>,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl ResvStation {
    /// Creates a new empty reservation station with given capacity.
    pub fn new(capacity: usize) -> ResvStation {
        ResvStation {
            capacity,
            contents: VecDeque::with_capacity(capacity),
        }
    }

    /// Reserves a new slot in the reservation station.
    /// TODO, Document, Types, Implement
    pub fn reserve(
        &mut self,
        instruction: Instruction,
        spec_bp_pc: usize,
        rob: &mut ReorderBuffer,
        rf: &mut RegisterFile,
    ) -> bool {
        if self.contents.len() + 1 >= self.capacity {
            return false
        }

        // Reserve a physical register for writeback.
        let mut name_rd = 0;
        match instruction.rd {
            Some(rd) => match rf.using_write(rd) {
                Some(n) => name_rd = n,
                None => return false, // No Available Physical Registers
            },
            None => (), // No need to rename as no writeback.
        }

        // Reserve a reorder buffer entry
        let rob_entry = match rob.reserve_entry(spec_bp_pc) {
            Some(entry) => entry,
            None => {
                rf.not_using_write(name_rd);
                return false
            },
        };

        self.contents.push_back(Reservation {
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
        });
        true
    }

    /// Consumes a reservation that follows the given criteria, if such a
    /// reservation exists.
    /// TODO, Document, Types, Implement
    pub fn consume(_criteria: ()) -> Option<Reservation> {
        unimplemented!()
    }
}

