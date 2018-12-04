use std::collections::VecDeque;

use either::Either;

use isa::op_code::Operation;
use isa::operand::Register;

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

    /// Returns whether the reservation station has free capacity to add more
    /// reservations.
    pub fn free_capactiy(&self) -> bool {
        self.contents.len() + 1 < self.capacity
    }

    /// Reserves an entry within the reservation station for future out of
    /// order execution. Returns whether or not the reservation was made
    /// successfully.
    pub fn reserve(&mut self, reservation: Reservation) -> Result<(),()> {
        if self.contents.len() + 1 >= self.capacity {
            return Err(())
        }
        self.contents.push_back(reservation);
        Ok(())
    }

    /// Consumes a reservation that follows the given criteria, if such a
    /// reservation exists.
    /// TODO, Document, Types, Implement
    pub fn consume(_criteria: ()) -> Option<Reservation> {
        unimplemented!()
    }
}

