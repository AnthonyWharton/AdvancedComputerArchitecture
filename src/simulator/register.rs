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
#[derive(Clone, Default)]
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
    // pub fn write_to_name(
    //     &mut self,
    //     rob: &mut ReorderBuffer,
    //     register: Register,
    //     rob_entry: usize,
    //     data: i32
    // ) {
    //     // Ensure we never write to the zero register.
    //     if name == 0 {
    //         return
    //     } else if name < 33 {
    //         self.file[name].data = data;
    //     } else {

    //         self.rob[name - 33].act_rd = data;
    //     }
    // }

    // /// Indicate that the given physical register file name with given
    // /// associated register is no longer needed for write operations, and flush
    // /// the value back to the architectural register file. If this is the
    // /// youngest rename of the register, this will reset the validity of the
    // /// architectural register file to true.
    // pub fn finished_write(&mut self, register: Register, name: usize) {
    //     // Register zero special case
    //     if register == Register::X0 {
    //         return;
    //     }

    //     let idx = register as usize;
    //     self.file[idx].data = self.physical[name - 33].data;
    //     if self.file[idx].rename == Some(name) {
    //         self.file[idx].rename = None;
    //     }
    //     self.remove_ref(name);
    // }
    
    /// _Safely_ renames the given register to the given reorder buffer entry.
    pub fn rename(&mut self, register: Register, rob_entry: usize) {
        // Register zero and the program counters are special cases
        match register {
            Register::X0 => return,
            Register::PC => return,
            _ => self.file[register as usize].rename = Some(rob_entry),
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
