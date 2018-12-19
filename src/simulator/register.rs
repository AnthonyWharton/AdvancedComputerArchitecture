use std::collections::VecDeque;
use std::default::Default;
use std::ops::{Index, IndexMut};

use crate::isa::operand::Register;

use super::reorder::ReorderBuffer;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The main register file, containing all the architectural registers and
/// logic for accessing, renaming, etc.
/// Registers `0..33` are the architectural registers, defined by
/// `Register as usize`, and `33..` are physical registers.
#[derive(Clone)]
pub struct RegisterFile {
    /// The architectural register lookup table.
    pub file: Vec<ArchRegEntry>,
}

/// The contents of a line in the Architectural Register File.
///
/// If the valid bit is not set (i.e. there is a valid rename), more up to date
/// information may be in the physical register file.
#[derive(Clone)]
pub struct ArchRegEntry {
    /// The latest committed value of the register.
    pub data: i32,
    /// The renamed name of the register in the physical register file. If this
    /// is not `None` then the architectural register entry is valid.
    pub rename: Option<usize>,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl Default for RegisterFile {
    fn default() -> RegisterFile {
        RegisterFile {
            file: vec![ArchRegEntry::default(); 33],
        }
    }
}

impl Index<Register> for RegisterFile {
    type Output = ArchRegEntry;

    fn index(&self, reg: Register) -> &Self::Output {
        &self.file[reg as usize]
    }
}

impl IndexMut<Register> for RegisterFile {
    fn index_mut(&mut self, reg: Register) -> &mut Self::Output {
        &mut self.file[reg as usize]
    }
}

impl RegisterFile {
    /// _Safely_ renames the given register to the given reorder buffer entry.
    pub fn rename(&mut self, register: Register, rob_entry: usize) {
        // Register zero and the program counters are special cases
        match register {
            Register::X0 => return,
            Register::PC => return,
            _ => self.file[register as usize].rename = Some(rob_entry),
        }
    }

    /// Writes back some data to a register entry, updating the rename/valid
    /// bit if applicable.
    pub fn writeback(&mut self, register: Register, rob_entry: usize, data: i32) {
        // Register zero special case
        if register == Register::X0 {
            return;
        }

        self[register].data = data;
        if Some(rob_entry) == self[register].rename {
            self[register].rename = None;
        }
    }

    /// Flushes the register file, this would happen when the pipeline is
    /// invalidated and needs to be restarted from scratch.
    pub fn flush(&mut self) {
        for reg in self.file.iter_mut() {
            reg.rename = None;
        }
    }
}

impl Default for ArchRegEntry {
    /// Returns the default register entry, that is with a valid value of 0
    /// set.
    fn default() -> ArchRegEntry {
        ArchRegEntry {
            data: 0,
            rename: None,
        }
    }
}
