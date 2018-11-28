use std::collections::VecDeque;

use isa::Instruction;

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
#[derive(Clone, Default)]
pub struct Reservation {
    /// The instruction being reserved.
    instr: Instruction,
    /// Whether the rs1 source register is ready. If no rs1 is required this
    /// will always be true.
    rs1_ready: bool,
    /// Whether the rs2 source register is ready. If no rs2 is required this
    /// will always be true.
    rs2_ready: bool,
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
    pub fn reserve() -> Option<()> {
        unimplemented!()
    }

    /// Consumes a reservation that follows the given criteria, if such a
    /// reservation exists.
    /// TODO, Document, Types, Implement
    pub fn consume(_criteria: ()) -> Option<Reservation> {
        unimplemented!()
    }
}

