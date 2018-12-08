use std::collections::VecDeque;
use std::default::Default;

use either::{Either, Left, Right};

use crate::isa::operand::Register;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// The main register file, containing all the architectural registers and
/// logic for accessing, renaming, etc.
/// Registers `0..33` are the architectural registers, defined by
/// `Register as usize`, and `33..` are physical registers.
#[derive(Clone)]
pub struct RegisterFile {
    /// The architectural register lookup table.
    arch: Vec<ArchRegEntry>,
    /// The physical registers that hold register data, and a count of how many
    /// references are making current use of it.
    physical: Vec<PhysicalRegEntry>,
    /// A queue of registers that are free for rename usage in the physical
    /// register file.
    free: VecDeque<usize>,
}

/// The contents of a line in the Architectural Register File.
///
/// If the valid bit is not set, more up to date information may be in the
/// physical register file.
#[derive(Clone)]
pub struct ArchRegEntry {
    /// The latest committed value of the register.
    data: i32,
    /// The 'valid' bit, i.e. the data is directly usable.
    valid: bool,
    /// The renamed name of the register in the physical register file, used
    /// when the valid bit is not set.
    rename: usize,
}

/// The contents of a line in the Phsyical Register File.
///
/// Will remain invalid until an execute unit writes into it.
#[derive(Clone, PartialEq)]
pub struct PhysicalRegEntry {
    /// The data stored in the line of the physical register file.
    data: i32,
    /// The 'valid' bit, i.e. the data is directly usable.
    valid: bool,
    /// The number of components that have a reference to this physical
    /// register entry.
    ref_count: u8,
}


///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl RegisterFile {
    /// Creates a new register file with specified amount of physical registers
    /// in the physical register file.
    pub fn new(physical_regs: usize) -> RegisterFile {
        RegisterFile {
            arch: vec![ArchRegEntry::default(); 33],
            physical: vec![PhysicalRegEntry::default(); physical_regs],
            free: (33 .. physical_regs + 33).collect(),
        }
    }

    /// Finds the value of a given register name if valid. If an architectural
    /// register file name is given, this will return the value of the
    /// architectural file's data regardless of validity. For a method that
    /// takes architectural register file validity into account use
    /// [`read_reg()`](#method.reg) instead.
    pub fn read_at_name(&self, name: usize) -> Option<i32> {
        if name < 33 {
            if self.arch[name].valid {
                Some(self.arch[name].data)
            } else {
                None
            }
        } else if self.physical[name - 33].valid {
            Some(self.physical[name - 33].data)
        } else {
            None
        }
    }

    /// Writes the give value to a given register name. If a physical register
    /// file name is given, the valid bit will be set to `true` upon a write.
    /// If an architectural register file name is given, the validity bit is
    /// not touched. To set the architectural register file validity bit,
    /// initiate a writeback with [`finished_write()`](#method.finished_write)
    /// instead.
    pub fn write_to_name(&mut self, name: usize, data: i32) {
        // Ensure we never write to the zero register.
        if 0 < name && name < 33 {
            self.arch[name].data = data;
        } else {
            self.physical[name - 33].data = data;
            self.physical[name - 33].valid = true;
        }
    }

    /// Finds the value of a given register if valid. This will follow the
    /// latest rename of that register if the valid bit is not set.
    ///
    /// Order of checks:
    ///   1) If, architectural register file entry is valid, return the data.
    ///   2) Else, follow the architectural register file rename to physical
    ///      register file.
    ///   3) If, physical register file entry is valid, return the data/
    ///   3) Else, no data is available for this name.
    pub fn read_reg(&self, reg: Register) -> Option<i32> {
        let name = reg as usize;
        if self.arch[name].valid {
            Some(self.arch[name].data)
        } else if self.physical[self.arch[name].rename].valid {
            Some(self.physical[self.arch[name].rename].data)
        } else {
            None
        }
    }

    /// If resources are available, will rename a register for future
    /// writeback. Renamed registers have no guarentee as to what will be
    /// inside of them when created.
    ///
    /// Returns the renamed register reference, if available, otherwise `None`
    /// is returned.
    pub fn using_write(&mut self, register: Register) -> Option<usize> {
        // Register zero and the program counters are special cases
        match register {
            Register::X0 => return Some(0),
            // TODO: Consider a more severe error for renaming program counter
            Register::PC => return None,
            _ => (),
        }

        let idx = register as usize;
        self.arch[idx].valid = false;
        match self.free.pop_front() {
            Some(name) => {
                self.physical[name - 33] = PhysicalRegEntry::default();
                self.arch[idx].rename = name;
                Some(name)
            },
            None => None,
        }
    }

    /// Used to free a renamed register if it was not actually needed. The
    /// renamed register must be completely untouched for this to be a valid
    /// operation.
    ///
    /// Returns true if the register was freed, and false otherwise.
    pub fn not_using_write(&mut self, name: usize) -> bool {
        if 33 < name && name < (self.physical.len() + 33) &&
           self.physical[name - 33] == PhysicalRegEntry::default()
        {
            self.free.push_front(name);
            true
        } else {
            false
        }
    }

    /// Indicate that the caller is intending to keep a reference to the given
    /// register. If the register is valid, this will have no effect, however
    /// if invalid (i.e. a renamed register), this will increment the reference
    /// count to the physical register file.
    pub fn using_read(&mut self, register: Register) -> Either<i32, usize> {
        let idx = register as usize;
        if self.arch[idx].valid {
            Left(self.arch[idx].data)
        } else {
            self.physical[self.arch[idx].rename - 33].ref_count += 1;
            Right(self.arch[idx].rename)
        }
    }

    /// Indicate that the given physical register file name with given
    /// associated register is no longer needed for write operations, and flush
    /// the value back to the architectural register file. If this is the
    /// youngest rename of the register, this will reset the validity of the
    /// architectural register file to true.
    pub fn finished_write(&mut self, register: Register, name: usize) {
        // Register zero special case
        if register == Register::X0 {
            return;
        }

        let idx = register as usize;
        self.arch[idx].data = self.physical[name - 33].data;
        if self.arch[idx].rename == name {
            self.arch[idx].valid = true;
        }
        self.remove_ref(name);
    }

    /// Indicate that the given physical register file name is no longer needed
    /// by something that had it as a read reference.
    pub fn finished_read(&mut self, name: usize) {
        self.remove_ref(name);
    }

    /// Removes a reference from the physical register file, and free's it if
    /// that was the last reference.
    fn remove_ref(&mut self, name: usize) {
        if name >= 33 {
            self.physical[name - 33].ref_count -= 1;
            if self.physical[name - 33].ref_count == 0 {
                self.free.push_back(name)
            }
        }
    }
}

impl Default for ArchRegEntry {
    /// Returns the default register entry, that is with a valid value of 0
    /// set.
    fn default() -> ArchRegEntry {
        ArchRegEntry {
            data: 0,
            valid: true,
            rename: 0,
        }
    }
}

impl Default for PhysicalRegEntry {
    /// Returns the default register entry, that is with a valid value of 0
    /// set.
    fn default() -> PhysicalRegEntry {
        PhysicalRegEntry {
            data: 0,
            valid: false,
            ref_count: 1,
        }
    }
}
