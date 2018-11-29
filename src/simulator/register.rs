use std::collections::VecDeque;
use std::default::Default;
use std::ops::{Index, IndexMut};

use either::{Either, Left, Right};

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
    /// The physical registers that hold register data, and a count of how many
    /// references are making current use of it.
    physical: Vec<(i32, u8)>,
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
    /// The name of the register in the physical register file, used when the
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
            &self.physical[name - 33].0
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
            &self.physical[self.arch[index as usize].rename - 33].0
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
            &mut self.physical[name - 33].0
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
            &mut self.physical[self.arch[index as usize].rename - 33].0
        }
    }
}

impl RegisterFile {
    /// Creates a new register file with specified amount of physical registers
    /// in the physical register file.
    pub fn new(physical_regs: usize) -> RegisterFile {
        RegisterFile {
            arch: vec![RegisterEntry::default(); 33],
            physical: Vec::with_capacity(physical_regs),
            free: (33 .. physical_regs + 33).collect(),
        }
    }

    /// If resources are available, will rename a register for future
    /// writeback. Renamed registers have no guarentee as to what will be
    /// inside of them when created.
    ///
    /// Returns the renamed register reference, if available, otherwise `None`
    /// is returned.
    pub fn using_write(&mut self, register: Register) -> Option<usize> {
        let idx = register as usize;
        self.arch[idx].valid = false;
        match self.free.pop_front() {
            Some(name) => {
                // No guarentee to contents, but we will wipe it to 0 for this
                // implementation.
                self.physical[name - 33] = (0, 1);
                self.arch[idx].rename = name;
                return Some(name)
            },
            None => return None,
        }
    }

    /// Indicate that the caller is intending to keep a reference to the given
    /// register. If the register is valid, this will have no effect, however
    /// if invalid (i.e. a renamed register), this will increment the reference
    /// count to the physical register file.
    pub fn using_read(&mut self, register: Register) -> Either<i32, usize> {
        let idx = register as usize;
        if self.arch[idx].valid {
            Left(self[register])
        } else {
            self.physical[self.arch[idx].rename - 33].1 += 1;
            Right(self.arch[idx].rename)
        }
    }

    /// Indicate that the given physical register file name with given
    /// associated register is no longer needed for write operations. If this
    /// is the youngest rename of the register, this will flush the value back
    /// into the architectural register file.
    pub fn finished_write(&mut self, register: Register, name: usize) {
        let idx = register as usize;
        if self.arch[idx].rename == name {
            self.arch[idx].data = self.physical[name - 33].0;
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
        self.physical[name - 33].1 -= 1;
        if self.physical[name - 33].1 <= 0 {
            self.free.push_back(name)
        }
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
