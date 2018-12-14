use std::ops::{Index, IndexMut};

use either::{Either, Left};

use crate::isa::operand::Register;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The reorder buffer is responsible for keeping an in-order list of
/// instructions that are being executed out of order, and their states. This
/// can then be used to 'commit' results back in order, when they are ready.
#[derive(Clone)]
pub struct ReorderBuffer {
    /// All the reorder buffer entries.
    rob: Vec<ReorderEntry>,
    /// A pointer to the start of the circular buffer.
    front: usize,
    /// A pointer to the end of the circular buffer.
    back: usize,
    /// The amount of items in the circular buffer.
    count: usize,
    /// The capacity of the circular buffer.
    capacity: usize,
}

/// The contents of a line in the Register File.
#[derive(Clone)]
pub struct ReorderEntry {
    /// The 'finished' bit, i.e. the data is directly usable, and the entry is
    /// ready for writeback.
    pub finished: bool,
    /// The program counter for this instruction, indicating the choice that
    /// the branch predictor made.
    pub pc: usize,
    /// The actual value of the Program Counter after execution. Only valid
    /// when finished is `true`.
    pub act_pc: usize,
    /// The actual value of the `rd` result register after execution. Only
    /// valid when finished is `true`.
    pub act_rd: i32,
    /// The pre-renamed `rd` result register.
    pub reg_rd: Option<Register>,
    /// The renamed `rd` result register.
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

impl ReorderBuffer {
    /// Creates a new reorder buffer with given capacity.
    pub fn new(capacity: usize) -> ReorderBuffer {
        ReorderBuffer {
            rob: vec![ReorderEntry::default(); capacity + 1],
            front: 0,
            back: 0,
            count: 0,
            capacity,
        }
    }

    /// If available, allocate a free entry in the reorder buffer with the
    /// (speculative) program counter chosen by the branch predictor for the
    /// instruction destined in the entry.
    pub fn reserve_entry(&mut self, pc: usize) -> Option<usize> {
        // Check we have space
        if self.count == self.capacity {
            return None;
        }

        let e = self.front;
        self.count += 1;
        self.front = (self.front + 1) % self.capacity;
        self.rob[e] = ReorderEntry::default();
        self.rob[e].pc = pc;
        Some(e)
    }

    /// Placeholder implementation.
    fn free_entry(&mut self) {
        if self.count == 0 {
            return;
        }
        self.count -= 1;
        self.back = (self.back + 1) % self.capacity;
    }
}

impl Index<usize> for ReorderBuffer {
    type Output = ReorderEntry;

    /// Access a reorder buffer entry. If an index is too large, it will wrap
    /// around to the 0th entry.
    fn index(&self, entry: usize) -> &ReorderEntry {
        &self.rob[entry % self.capacity]
    }
}

impl IndexMut<usize> for ReorderBuffer {
    /// Mutably access a reorder buffer entry. If an index is too large, it
    /// will wrap around to the 0th entry.
    fn index_mut(&mut self, entry: usize) -> &mut ReorderEntry {
        &mut self.rob[entry % self.capacity]
    }
}

impl Default for ReorderEntry {
    /// Creates an unfinished and unpopulated reorder buffer entry.
    fn default() -> ReorderEntry {
        ReorderEntry {
            finished: false,
            pc: 0,
            act_pc: 0,
            act_rd: 0,
            reg_rd: None,
            name_rd: None,
            rs1: Left(0),
            rs2: Left(0),
            imm: None,
        }
    }
}
