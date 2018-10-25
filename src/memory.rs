use std::ops::{Deref, DerefMut};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use elf::{File, Section, ParseError};
use elf::types::*;

use util::config::Config;
use util::exit::Exit::{FileLoadError, ElfError};

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

const INIT_MEMORY_SIZE: usize = 1_000_000; // 1 Megabyte

///////////////////////////////////////////////////////////////////////////////
//// TYPES

/// Type alias for an individual word in the machine.
pub type Word = i32;

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

impl Memory {
    /// Creates a new `Memory` struct of given capacity with a 0-initialised
    /// byte-data.
    pub fn create_empty(capacity: usize) -> Memory {
        Memory(vec!(0u8; capacity))
    }

    /// Reads a word from `Memory` at a given index, returning the word and
    /// whether or not a misaligned access was used.
    pub fn read_word(&mut self, index: usize) -> (Word, bool) {
        // Check if memory data structure is large enough, if not extend
        let (diff, sufficient) = (index + 3).overflowing_sub(self.len());
        if !sufficient {
            self.0.append(&mut vec!(0; diff));
        }

        // Read 4 bytes to make a word
        let mut rdr = &self.0[index..];
        (rdr.read_i32::<LittleEndian>().unwrap(), index % 4 == 0)
    }

    /// Writes a word to `Memory` at a given index, returning and
    /// whether or not a misaligned access was used.
    pub fn write_word(&mut self, index: usize, word: Word) -> bool {
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
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Loads the elf file into a Memory data structure.
/// TODO: Change this to load into some yet-to-be-defined state struct (as
/// there will be registers and other gubbins to initialise).
pub fn load_elf(config: &Config) -> Memory {
    let file = match File::open_path(&config.elf_file) {
        Ok(f)  => f,
        Err(e) => match e {
            ParseError::IoError(ee)  => FileLoadError
                .exit(Some(&format!("Failed to load elf file:\n{}", ee))),
            ParseError::InvalidMagic => FileLoadError
                .exit(Some("That's no elf file! (Invalid Magic)")),
            ParseError::InvalidFormat(ee) => FileLoadError
                .exit(Some(&format!("Invalid Format! {:?}", ee))),
            ParseError::NotImplemented => FileLoadError
                .exit(Some("Something went wrong loading the elf file.")),
        },
    };

    // Verify headers, these will quit the program on a failure.
    verify_file_header(&file.ehdr);
    for h in file.phdrs.iter() {
        verify_prog_header(h);
    }

    // Initialise and load in memory
    let mut mem: Memory = Memory::create_empty(INIT_MEMORY_SIZE);
    for s in file.sections.iter() {
        load_section(&mut mem, s);
    }

    mem
}

/// Verifies the given ELF file header is compatible with the simulator, and 
/// quits if invalid. If this function returns, it can be assumed that the 
/// header is good to go!
fn verify_file_header(header: &FileHeader) {
    if header.class != ELFCLASS32 { 
        ElfError.exit(Some("Found 64 bit ELF file, expected 32 bit."));
    }
    if header.data != ELFDATA2LSB {
        ElfError.exit(Some("Found Big Endian ELF file, expected Little Endian."));
    }
    if header.version != EV_CURRENT {
        ElfError.exit(Some("Incompatible ELF file version, expected 1.")); 
    }
    if header.osabi != ELFOSABI_SYSV {
        ElfError.exit(Some("Incompatible OS ABI in ELF file header, expected Unix - System V."));
    }
    if header.elftype != ET_EXEC { 
        ElfError.exit(Some("Incompatible object file type in ELF file header, expected EXEC."));
    }
    if header.machine != Machine(0xf3) {
        ElfError.exit(Some("Incompatible ISA in ELF file header, expected RISC-V."));
    }
}

/// Loose checks to make sure that an _individual_ program header is not
/// something that should break the simulator (e.g. dynamically linked libs),
/// and quits the simulator if invalid. If this function returns, it can be
/// assumed that the header is good to go!
fn verify_prog_header(header: &ProgramHeader) {
    match header.progtype {
        PT_NULL | PT_LOAD | PT_NOTE | PT_PHDR => (),
        _ => ElfError.exit(Some("Elf file contained unsupported program header type.")),
    }
}

/// Loads the data from the given section into memory if required. If not
/// required, performs no operation.
fn load_section(memory: &mut Memory, section: &Section) {
    // Check if we actually want to load this section
    if section.shdr.name == ".shstrtab" { return }
    if section.shdr.size == 0 { return }

    // Check if we need to expand memory
    // `usize as u64` cast is safe as simulator is for 32 bit architectures
    let (extra, sufficient_mem) = (section.shdr.addr + section.shdr.size)
                                  .overflowing_sub(memory.capacity() as u64);
    if !sufficient_mem {
        memory.reserve(extra as usize);
    }

    // Load in the section
    // `usize as u64` cast is safe as simulator is for 32 bit architectures
    let s_addr: usize = section.shdr.addr as usize;
    let e_addr: usize = s_addr + section.data.len();
    memory.splice(s_addr..e_addr, section.data.iter().cloned());
}

