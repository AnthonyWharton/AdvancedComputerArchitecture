use std::cmp::min;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result as FmtResult};

use either::{Either, Left, Right};

use crate::isa::op_code::Operation;
use crate::isa::operand::Register;

use super::execute::{ExecuteUnit, ExecutionLen, UnitType};
use super::reorder::ReorderBuffer;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The reservation station is the cache between the decode and dispatch stage.
/// It is responsible for holding instructions that are ready, or waiting for
/// dependencies, before they are dispatched to execution units.
#[derive(Clone, Debug)]
pub struct ResvStation {
    /// The amount of reservations the Reservation Station can hold.
    pub capacity: usize,
    /// The contents of the Reservation Station.
    pub contents: VecDeque<Reservation>,
}

/// A single Reservation within the Reservation Station.
#[derive(Clone, Debug)]
pub struct Reservation {
    /// The entry in the reorder buffer that corresponds to this entry.
    pub rob_entry: usize,
    /// The pending operation
    pub op: Operation,
    /// The program counter value for this instruction, indicating the choice
    /// the branch predictor made.
    pub pc: usize,
    /// The pending writeback register.
    pub reg_rd: Option<Register>,
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
    pub fn free_capacity(&self) -> bool {
        self.contents.len() < self.capacity
    }

    /// Reserves an entry within the reservation station for future out of
    /// order execution. Returns whether or not the reservation was made
    /// successfully.
    pub fn reserve(&mut self, reservation: Reservation) -> Result<(), ()> {
        if self.contents.len() >= self.capacity {
            return Err(());
        }
        self.contents.push_back(reservation);
        Ok(())
    }

    /// Consumes the next reservation station entry that is ready for
    /// execution, and is supported by the given execution unit type. The limit
    /// field reduces how many entries of the reservation station will be
    /// checked.
    pub fn consume_next(
        &self,
        new_rs: &mut ResvStation,
        eu: &ExecuteUnit,
        rob: &ReorderBuffer,
        limit: usize,
    ) -> (Option<Reservation>, usize) {
        let act_limit = if limit == 0 {
            self.contents.len()
        } else {
            min(limit, self.contents.len())
        };
        let unit_type = eu.get_type();
        let next_valid = new_rs
            .contents
            .iter()
            .cloned()
            .take(act_limit)
            .enumerate()
            .find(|(_, r)| {
                // Check operation is supported by execute unit type
                unit_type == UnitType::from(r.op)
                &&
                // Check execute unit is free
                eu.is_free(ExecutionLen::from(r.op))
                &&
                // Check rs1 is ready
                match r.rs1 {
                    Left(_)  => true,
                    Right(n) => rob[n].act_rd.is_some(),
                }
                // Check rs2 is ready
                &&
                match r.rs2 {
                    Left(_)  => true,
                    Right(n) => rob[n].act_rd.is_some(),
                }
            });

        // Consume the reservation, if a valid one was found.
        match next_valid {
            Some((idx, _)) => (new_rs.contents.remove(idx), act_limit - 1),
            None => (None, act_limit),
        }
    }

    /// Flushes the reservation station, this would happen when the pipeline is
    /// invalidated and needs to be restarted from scratch.
    pub fn flush(&mut self) {
        self.contents.clear()
    }
}

impl Display for Reservation {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:2}", self.rob_entry)?;
        write!(f, " {:>6}", self.op)?;
        write!(f, " {:08x}", self.pc)?;
        write!(f, " {}", format_option!("{:#}", self.reg_rd))?;
        match self.rs1 {
            Left(val) => write!(f, " v{}", val)?,
            Right(rob) => write!(f, " r{}", rob)?,
        }
        match self.rs2 {
            Left(val) => write!(f, " v{}", val)?,
            Right(rob) => write!(f, " r{}", rob)?,
        }
        write!(f, " {}", format_option!("{}", self.imm))?;
        Ok(())
    }
}
