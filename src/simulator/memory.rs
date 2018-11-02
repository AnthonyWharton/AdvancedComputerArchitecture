
use std::ops::{Deref, DerefMut};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use elf::Section;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Smart Pointer on a vector of bytes to store the memory for the simulator.
/// See the implemented methods for extra functionality.
pub struct Memory(Vec<u8>);

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

/// Allows for direct access to the memory data structure nested within the
/// `Memory` struct.
impl Deref for Memory {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

/// Allows for direct mutable access to the memory data structure nested within
/// the `Memory` struct.
impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

#[allow(dead_code)]
impl Memory {
    /// Creates a new `Memory` struct of given capacity with a 0-initialised
    /// byte-data.
    pub fn create_empty(capacity: usize) -> Memory {
        Memory(vec!(0u8; capacity))
    }

    /// Reads a signed 32 bit word from `Memory` at a given index, returning
    /// the word and whether or not a misaligned access was used.
    pub fn read_i32(&mut self, index: usize) -> (i32, bool) {
        // Check if memory data structure is large enough, if not extend
        let (diff, sufficient) = (index + 3).overflowing_sub(self.len());
        if !sufficient {
            self.0.append(&mut vec!(0; diff));
        }

        // Read 4 bytes to make an i32
        let mut rdr = &self.0[index..];
        (rdr.read_i32::<LittleEndian>().unwrap(), index % 4 == 0)
    }

    /// Writes a signed 32 bit word to `Memory` at a given index, returning
    /// whether or not a misaligned access was used.
    pub fn write_i32(&mut self, index: usize, word: i32) -> bool {
        // Check if memory data structure is large enough, if not extend
        let (diff, sufficient) = (index + 3).overflowing_sub(self.len());
        if !sufficient {
            self.0.append(&mut vec!(0; diff));
        }

        // Write 4 bytes at the given index
        let mut wtr = &mut self.0[index..];
        wtr.write_i32::<LittleEndian>(word).unwrap();
        index % 4 == 0
    }

    /// Reads a signed 16 bit half-word from `Memory` at a given index, 
    /// returning the half-word and whether or not a misaligned access was
    /// used.
    pub fn read_i16(&mut self, index: usize) -> (i16, bool) {
        // Check if memory data structure is large enough, if not extend
        let (diff, sufficient) = (index + 1).overflowing_sub(self.len());
        if !sufficient {
            self.0.append(&mut vec!(0; diff));
        }

        // Read 2 bytes to make a i16
        let mut rdr = &self.0[index..];
        (rdr.read_i16::<LittleEndian>().unwrap(), index % 2 == 0)
    }
    
    /// Reads an unsigned 16 bit half-word from `Memory` at a given index, 
    /// returning the half-word and whether or not a misaligned access was
    /// used.
    pub fn read_u16(&mut self, index: usize) -> (u16, bool) {
        let r = self.read_i16(index);
        return (r.0 as u16, r.1);
    }

    /// Writes a signed 16 bit half-word to `Memory` at a given index, 
    /// returning whether or not a misaligned access was used.
    pub fn write_i16(&mut self, index: usize, word: i16) -> bool {
        // Check if memory data structure is large enough, if not extend
        let (diff, sufficient) = (index + 1).overflowing_sub(self.len());
        if !sufficient {
            self.0.append(&mut vec!(0; diff));
        }

        // Write 2 bytes at the given index
        let mut wtr = &mut self.0[index..];
        wtr.write_i16::<LittleEndian>(word).unwrap();
        index % 2 == 0
    }

    /// Loads the data from the given section into memory if required. If not
    /// required, performs no operation.
    pub fn load_elf_section(&mut self, section: &Section) {
        // Check if we actually want to load this section
        if section.shdr.name == ".shstrtab" { return }
        if section.shdr.size == 0 { return }
    
        // Check if we need to expand memory
        // `usize as u64` cast is safe as simulator is for 32 bit architectures
        let (extra, sufficient_mem) = (section.shdr.addr + section.shdr.size)
                                      .overflowing_sub(self.capacity() as u64);
        if !sufficient_mem {
            self.reserve(extra as usize);
        }
    
        // Load in the section
        // `usize as u64` cast is safe as simulator is for 32 bit architectures
        let s_addr: usize = section.shdr.addr as usize;
        let e_addr: usize = s_addr + section.data.len();
        self.splice(s_addr..e_addr, section.data.iter().cloned());
    }
}

