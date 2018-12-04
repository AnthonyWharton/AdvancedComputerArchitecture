use std::fmt::{Display, Formatter, LowerHex, Result};
use std::ops::{Deref, DerefMut};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use elf::Section;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

pub const INIT_MEMORY_SIZE: usize = 1_000_000; // 1 Megabyte

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Container for a memory access.
#[derive(Copy, Clone, Debug)]
pub struct Access<W> {
    /// Whether or not the access was aligned.
    pub aligned: bool,
    /// The word as a result of the memory access.
    pub word: W,
}

/// Smart Pointer on a vector of bytes to store the memory for the simulator.
/// See the implemented methods for extra functionality.
#[derive(Clone)]
pub struct Memory(Vec<u8>);

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

/// Implementation to pretty print a memory address access and whether or not
/// it was aligned.
impl<W: Display + LowerHex> Display for Access<W> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.aligned {
            write!(f, "{:08x} (aligned)", self.word)
        } else {
            write!(f, "{:08x} (misaligned)", self.word)
        }
    }
}

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
    ///
    /// Requires self to be mutable as this function will 0-extend memory if
    /// attempting to access memory that has not been initialised before.
    pub fn read_i32(&self, index: usize) -> Access<i32> {
        Access {
            aligned: index % 4 == 0,
            word: if self.is_capable(index, 4) {
                (&self.0[index..]).read_i32::<LittleEndian>().unwrap()
            } else {
                0
            },
        }
    }

    /// Writes a signed 32 bit word to `Memory` at a given index, returning
    /// whether or not a misaligned access was used.
    ///
    /// Requires self to be mutable as this function will 0-extend memory if
    /// attempting to access memory that has not been initialised before.
    pub fn write_i32(&mut self, index: usize, word: i32) -> bool {
        self.zero_extend(index);

        let mut wtr = &mut self.0[index..];
        wtr.write_i32::<LittleEndian>(word).unwrap();
        index % 4 == 0
    }

    /// Reads a signed 16 bit half-word from `Memory` at a given index,
    /// returning the half-word and whether or not a misaligned access was
    /// used.
    ///
    /// Requires self to be mutable as this function will 0-extend memory if
    /// attempting to access memory that has not been initialised before.
    pub fn read_i16(&self, index: usize) -> Access<i16> {
        Access {
            aligned: index % 2 == 0,
            word: if self.is_capable(index, 2) {
                (&self.0[index..]).read_i16::<LittleEndian>().unwrap()
            } else {
                0
            },
        }
    }

    /// Reads an unsigned 16 bit half-word from `Memory` at a given index,
    /// returning the half-word and whether or not a misaligned access was
    /// used.
    ///
    /// Requires self to be mutable as this function will 0-extend memory if
    /// attempting to access memory that has not been initialised before.
    pub fn read_u16(&mut self, index: usize) -> Access<u16> {
        let r = self.read_i16(index);
        Access {
            aligned: r.aligned,
            word: r.word as u16,
        }
    }

    /// Writes a signed 16 bit half-word to `Memory` at a given index,
    /// returning whether or not a misaligned access was used.
    ///
    /// Requires self to be mutable as this function will 0-extend memory if
    /// attempting to access memory that has not been initialised before.
    pub fn write_i16(&mut self, index: usize, word: i16) -> bool {
        self.zero_extend(index + 1);

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

        // Extend the size of memory to contain new data
        self.zero_extend((section.shdr.addr + section.shdr.size) as usize);

        // Load in the section
        // `usize as u64` cast is safe as simulator is for 32 bit architectures
        let s_addr: usize = section.shdr.addr as usize;
        let e_addr: usize = s_addr + section.data.len();
        self.splice(s_addr..e_addr, section.data.iter().cloned());
    }

    /// Zero extends memory to the index given, if it is not currently
    /// generated within the simulated `Memory` data structure.
    fn zero_extend(&mut self, index: usize) {
        // Check if memory data structure is large enough, if not extend
        let (diff, sufficient) = (index).overflowing_sub(self.len());
        if !sufficient {
            self.0.append(&mut vec!(0; diff));
        }
    }

    /// Whether or not the memory is capable of reading or writing a value of
    /// `size` bytes at `index` - i.e. if the memory has been allocated on the
    /// host machine of the simulator.
    fn is_capable(&self, index: usize, size: usize) -> bool {
        if size == 0 {
            true
        } else {
            (index + size - 1).overflowing_sub(self.len()).1
        }
    }

}

