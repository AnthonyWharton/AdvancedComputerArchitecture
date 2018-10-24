use elf::File;
use elf::Section;
use elf::ParseError;
use elf::types::*;

use util::config::Config;
use util::exit::Exit::{FileLoadError, ElfError};

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

const INIT_MEMORY_SIZE: usize = 1_000_000; // 1 Megabyte

///////////////////////////////////////////////////////////////////////////////
//// TYPES

/// Type alias for the data structure that holds main memory
// type Memory = Box<Vec<u8>>;
type Memory = Vec<u8>;

/// Type alias for an individual word in the machine.
type Word = u32;

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
    // let mut mem: Memory = Box::new(vec!(0u8; INIT_MEMORY_SIZE));
    let mut mem: Memory = vec!(0u8; INIT_MEMORY_SIZE);
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

