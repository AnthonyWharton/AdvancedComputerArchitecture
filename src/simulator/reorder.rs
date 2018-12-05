use std::collections::VecDeque;

use either::{Either, Left};

use isa::operand::Register;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The reorder buffer is responsible for keeping an in-order list of
/// instructions that are being executed out of order, and their states. This
/// can then be used to 'commit' results back in order, when they are ready.
#[derive(Clone)]
pub struct ReorderBuffer {
    rob: Vec<ReorderEntry>,
    free: VecDeque<usize>,
}

/// The contents of a line in the Register File.
#[derive(Clone)]
pub struct ReorderEntry {
    /// The 'finished' bit, i.e. the data is directly usable, and the entry is
    /// ready for writeback.
    pub finished: bool,
    /// The speculative program counter for this instruction from the branch
    /// prediction unit. Should the Program Counter be different to this field
    /// at writeback, a branch misprediction has occured.
    pub spec_bp_pc: usize,
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
            free: (0 .. capacity).collect(),
        }
    }

    /// If available, allocate a free entry in the reorder buffer with the
    /// speculative program counter chosen by the branch predictor.
    pub fn reserve_entry(&mut self, spec_bp_pc: usize) -> Option<usize> {
        match self.free.pop_front() {
            Some(e) => {
                self.rob[e] = ReorderEntry::default();
                self.rob[e].spec_bp_pc = spec_bp_pc;
                Some(e)
            },
            None => None,
        }
    }
}

impl Default for ReorderEntry {
    /// Creates an unfinished and unpopulated reorder buffer entry.
    fn default() -> ReorderEntry {
        ReorderEntry {
            finished: false,
            spec_bp_pc: 0,
            reg_rd: None,
            name_rd: None,
            rs1: Left(0),
            rs2: Left(0),
            imm: None,
        }
    }
}
