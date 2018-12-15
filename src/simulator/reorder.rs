use std::ops::{Index, IndexMut};

use either::{Either, Left};

use crate::isa::op_code::Operation;
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
    /// The operation that executed
    pub op: Operation,
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
            rob: vec![ReorderEntry::default(); capacity],
            front: 0,
            back: 0,
            count: 0,
            capacity,
        }
    }

    /// Returns whether the reorder buffer has free capacity to allocate more
    /// entries.
    pub fn free_capacity(&self) -> bool {
        self.count < self.capacity
    }

    /// If available, reserves a slot for a given reorder buffer entry.
    pub fn reserve_entry(&mut self, entry: ReorderEntry) -> Option<usize> {
        // Check we have space
        if self.count >= self.capacity {
            return None;
        }

        let e = self.back;
        self.count += 1;
        self.back = (self.back + 1) % self.capacity;
        self.rob[e] = entry;
        Some(e)
    }

    /// If finished, pops the front ready entries off of the reorder buffer. If
    /// an empty Vec is returned, no entries have finished execution.
    /// Modifications are only made to the new reorder buffer.
    pub fn pop_finished_entries(
        &self,
        new_rob: &mut ReorderBuffer,
        finish_rob_entry: Option<usize>,
    ) -> (Vec<ReorderEntry>, bool) {
        if self.count == 0 {
            return (vec![], false);
        }

        if self.rob[self.front].finished {
            new_rob.count -= 1;
            new_rob.front = (self.front + 1) % self.capacity;

            (
                vec![new_rob.rob[self.front].clone()],
                self.front == finish_rob_entry.unwrap_or(self.capacity + 1)
            )
        } else {
            (vec![], false)
        }
    }

    /// Flushes the reorder buffer, this would happen when the pipeline is
    /// invalidated and needs to be restarted from scratch.
    pub fn flush(&mut self) {
        self.front = 0;
        self.back = 0;
        self.count = 0;
    }
}

impl Index<usize> for ReorderBuffer {
    type Output = ReorderEntry;

    /// Absolute access to a reorder buffer entry. If an index is too large, it
    /// will wrap around to the 0th entry.
    fn index(&self, entry: usize) -> &ReorderEntry {
        &self.rob[entry % self.capacity]
    }
}

impl IndexMut<usize> for ReorderBuffer {
    /// Absolute mutable access to a reorder buffer entry. If an index is too
    /// large, it will wrap around to the 0th entry.
    fn index_mut(&mut self, entry: usize) -> &mut ReorderEntry {
        &mut self.rob[entry % self.capacity]
    }
}

impl Default for ReorderEntry {
    /// Creates an unfinished and unpopulated reorder buffer entry.
    fn default() -> ReorderEntry {
        ReorderEntry {
            finished: false,
            op: Operation::ADDI,
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
