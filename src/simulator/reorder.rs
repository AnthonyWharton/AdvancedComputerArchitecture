use std::cmp::min;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

use either::{Either, Left, Right};

use crate::isa::op_code::Operation;
use crate::isa::operand::Register;

use super::branch::ReturnStackOp;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The reorder buffer is responsible for keeping an in-order list of
/// instructions that are being executed out of order, and their states. This
/// can then be used to 'commit' results back in order, when they are ready.
#[derive(Clone, Debug)]
pub struct ReorderBuffer {
    /// All the reorder buffer entries.
    pub rob: Vec<ReorderEntry>,
    /// A pointer to the start of unfinished entries in the circular buffer.
    pub front_fin: usize,
    /// A pointer to the start of the circular buffer.
    pub front: usize,
    /// A pointer to the end of the circular buffer.
    pub back: usize,
    /// The amount of items in the circular buffer.
    pub count: usize,
    /// The capacity of the circular buffer.
    pub capacity: usize,
}

/// The contents of a line in the Register File.
#[derive(Clone, Debug)]
pub struct ReorderEntry {
    /// The 'finished' bit, i.e. the data is directly usable, and the entry is
    /// ready for writeback.
    pub finished: bool,
    /// The number of components that have a reference to this reorder buffer
    /// entry.
    pub ref_count: u8,
    /// What this instruction has done to the return stack (used as feedback
    /// for the branch predictor).
    pub rs_operation: ReturnStackOp,
    /// The operation that executed
    pub op: Operation,
    /// The program counter for this instruction, indicating the choice that
    /// the branch predictor made.
    pub pc: usize,
    /// The actual value of the Program Counter after execution. Only valid
    /// when finished is `true`.
    pub act_pc: i32,
    /// The actual value of the `rd` result register after execution. Only
    /// valid when finished is `true`.
    pub act_rd: Option<i32>,
    /// The pre-renamed `rd` result register.
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

impl ReorderBuffer {
    /// Creates a new reorder buffer with given capacity.
    pub fn new(capacity: usize) -> ReorderBuffer {
        ReorderBuffer {
            rob: vec![ReorderEntry::default(); capacity],
            front_fin: 0,
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

    /// Recieves a bypass result from an execute unit and then adds it to the
    /// relevant reorder entries.
    pub fn execute_bypass(&mut self, entry: usize, result: i32) {
        let mut i = entry;
        while i != self.back {
            if let Right(n) = self[i].rs1 {
                if n == entry {
                    self[i].rs1 = Left(result);
                    self[entry].ref_count -= 1;
                }
            }
            if let Right(n) = self[i].rs2 {
                if n == entry {
                    self[i].rs2 = Left(result);
                    self[entry].ref_count -= 1;
                }
            }
            i = (i + 1) % self.capacity;
        }
    }

    /// If finished, pops the front ready entries off of the reorder buffer. If
    /// an empty Vec is returned, no entries have finished execution.
    /// Modifications are only made to the new reorder buffer.
    pub fn pop_finished_entries(
        &self,
        new_rob: &mut ReorderBuffer,
        limit: usize,
    ) -> Vec<usize> {
        if self.count == 0 {
            return vec![]
        }

        let mut popped = vec![];
        let unfinished_count = if self.back < self.front_fin {
            self.back + self.capacity - self.front_fin
        } else if self.front_fin < self.back {
            self.back - self.front_fin
        } else { // self.front_fin == self.back
            self.count
        };
        for i in 0..min(limit, unfinished_count) {
            if self.rob[(self.front_fin + i) % self.capacity].finished {
                new_rob.front_fin = (new_rob.front_fin + 1) % new_rob.capacity;
                new_rob.cleanup();
                popped.push((self.front_fin + i) % self.capacity)
            } else {
                for _ in i .. min(limit, unfinished_count) {
                    new_rob.cleanup();
                }
                break;
            }
        }

        popped
    }

    /// Cleans up any straggling entries that are finished _and_ have a zero
    /// reference count.
    fn cleanup(&mut self) {
        if self.rob[self.front].finished && self.rob[self.front].ref_count == 0 {
            let new_front = if self.front != self.front_fin {
                (self.front + 1) % self.capacity
            } else {
                self.front_fin
            };
            if new_front != self.front_fin {
                self.count -= 1;
                self.front = new_front;
            }
        }
    }

    /// Flushes the reorder buffer, this would happen when the pipeline is
    /// invalidated and needs to be restarted from scratch.
    pub fn flush(&mut self) {
        self.front_fin = 0;
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
            ref_count: 0,
            rs_operation: ReturnStackOp::None,
            op: Operation::ADDI,
            pc: 0,
            act_pc: 0,
            act_rd: None,
            reg_rd: None,
            rs1: Left(0),
            rs2: Left(0),
            imm: None,
        }
    }
}

impl Display for ReorderEntry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", if self.finished { "✓" } else { "×" })?;
        write!(f, " {}", match self.rs_operation {
            ReturnStackOp::None => "N",
            ReturnStackOp::Pushed(_) => "U",
            ReturnStackOp::Popped => "O",
            ReturnStackOp::PushPop(_) => "B"
        })?;
        write!(f, " {:2}", self.ref_count)?;
        write!(f, " {:>6}", self.op)?;
        write!(f, " {:08x}", self.pc)?;
        write!(f, " {:08x}", self.act_pc)?;
        write!(f, " {}", format_option!("{}", self.act_rd))?;
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
