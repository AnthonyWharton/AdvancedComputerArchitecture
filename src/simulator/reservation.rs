use std::collections::VecDeque;

use either::{Either, Left, Right};

use isa::op_code::Operation;
use isa::operand::Register;
use super::execute::UnitType;
use super::register::RegisterFile;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The reservation station is the cache between the decode and dispatch stage.
/// It is responsible for holding instructions that are ready, or waiting for
/// dependencies, before they are dispatched to execution units.
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
    pub pc: usize,
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

    pub fn consume_next(
        &mut self,
        unit_type: UnitType,
        rf: &RegisterFile,
        limit: usize,
    ) -> Option<Reservation> {
        let act_limit = if limit == 0 {
            self.contents.len()
        } else {
            limit
        };
        let next_valid = self.contents.iter()
                                      .cloned()
                                      .take(act_limit)
                                      .enumerate()
                                      .find(|(_, r)| {
            // Unit does not require this type of instruction
            unit_type == UnitType::from(r.op)
            &&
            match r.rs1 {
                Left(_)  => true,
                Right(n) => rf.read_at_name(n).is_some(),
            }
            &&
            match r.rs2 {
                Left(_)  => true,
                Right(n) => rf.read_at_name(n).is_some(),
            }
        });

        // Consume the reservation, if a valid one was found.
        match next_valid {
            Some((idx, _)) => self.contents.remove(idx),
            None           => None,
        }
    }
}

