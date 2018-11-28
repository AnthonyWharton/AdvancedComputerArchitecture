use std::collections::VecDeque;
use std::default::Default;
use std::ops::{Index, IndexMut};

use isa::operand::Register;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The main register file, containing all the architectural registers and
/// logic for accessing, renaming, etc.
/// Registers `0..33` are the architectural registers, defined by
/// `Register as usize`, and `33..` are physical registers.
#[derive(Clone)]
pub struct RegisterFile {
    /// The architectural register lookup table.
    arch: Vec<RegisterEntry>,
    /// The physical registers that hold register data.
    phsyical: Vec<i32>,
    /// A queue of registers that are free for rename usage in the physical
    /// register file.
    free: VecDeque<usize>,
}

/// The contents of a line in the Register File.
#[derive(Clone)]
pub struct RegisterEntry {
    /// If valid bit is set, this contains value of the architectural Register,
    /// else is contains the last valid value of the architectural Register.
    data: i32,
    /// The 'valid' bit, i.e. the data is directly usable.
    valid: bool,
    /// The name of the register in the phsyical register file, used when the
    /// valid bit is not set.
    rename: usize,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

/// Implementation used for indexing the register file by the raw register
/// name, with **NO** guarentee for validity. (If an architectural register is
/// given, it will follow the most recent rename for it).
impl Index<usize> for RegisterFile {
    type Output = i32;

    /// Returns the register with the given name.
    fn index(&self, name: usize) -> &i32 {
        if name < 33 {
            &self[Register::from(name as i32)]
        } else {
            &self.phsyical[name - 33]
        }
    }
}

/// Implementation used for indexing the register file by architectural
/// register. Will use the most recent renamed version of the register, with
/// **NO** guarentee for validity.
impl Index<Register> for RegisterFile {
    type Output = i32;

    fn index(&self, index: Register) -> &i32 {
        if self.arch[index as usize].valid {
            &self.arch[index as usize].data
        } else {
            &self.phsyical[self.arch[index as usize].rename - 33]
        }
    }
}

/// Implementation used for mutably indexing the register file by the raw
/// register name, with **NO** guarentee for validity. (If an architectural
/// register is given, it will follow the most recent rename for it).
impl IndexMut<usize> for RegisterFile {
    /// Returns the register with the given name.
    fn index_mut(&mut self, name: usize) -> &mut i32 {
        if name < 33 {
            &mut self[Register::from(name as i32)]
        } else {
            &mut self.phsyical[name - 33]
        }
    }
}

/// Implementation used for mutably indexing the register file by architectural
/// register. Will use the most recent renamed version of the register, with
/// **NO** guarentee for validity.
impl IndexMut<Register> for RegisterFile {
    fn index_mut(&mut self, index: Register) -> &mut i32 {
        if self.arch[index as usize].valid {
            &mut self.arch[index as usize].data
        } else {
            &mut self.phsyical[self.arch[index as usize].rename - 33]
        }
    }
}

impl RegisterFile {
    /// Creates a new register file with specified amount of physical registers
    /// in the phsyical register file.
    pub fn new(physical_regs: usize) -> RegisterFile {
        RegisterFile {
            arch: vec![RegisterEntry::default(); 33],
            phsyical: Vec::with_capacity(physical_regs),
            free: (33..physical_regs+33).collect(),
        }
    }

    pub fn rename(&mut self, register: Register) -> bool {
        self.arch[register as usize].valid = false;
        match self.free.pop_front() {
            Some(name) => self.arch[register as usize].rename = name,
            None => return false,
        }
        return true
    }
}

impl Default for RegisterEntry {
    /// Returns the default register entry, that is with a valid value of 0
    /// set.
    fn default() -> RegisterEntry {
        RegisterEntry {
            data: 0,
            valid: true,
            rename: 0,
        }
    }
}
