use std::collections::VecDeque;

use isa::operand::Register;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

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
    finished: bool,
    /// The speculative program counter for this instruction from the branch
    /// prediction unit. Should the Program Counter be different to this field
    /// at writeback, a branch misprediction has occured.
    spec_bp_pc: usize,
    /// The pre-renamed `rd` result register.
    reg_rd: Option<Register>,
    /// The renamed `rd` result register.
    name_rd: Option<usize>,
    /// The name of the `rs1` register.
    name_rs1: Option<usize>,
    /// The name of the `rs2` register.
    name_rs2: Option<usize>,
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
            name_rs1: None,
            name_rs2: None,
        }
    }
}
