use elf::{File, ParseError};
use elf::types::{Machine, FileHeader, ProgramHeader, ELFCLASS32, ELFDATA2LSB,
    EV_CURRENT, ELFOSABI_SYSV, ET_EXEC, PT_NULL, PT_LOAD, PT_NOTE, PT_PHDR};

use isa::operand::Register;
use simulator::state::State;
use super::config::Config;
use super::exit::Exit::{FileLoadError, ElfError};

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Loads the elf file into a Memory data structure.
pub fn load_elf(config: &Config) -> State {
    let file: File = match File::open_path(&config.elf_file) {
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

    // Declare state and memory
    let mut state  = State::default();

    // Initialise and load in memory
    for s in file.sections.iter() {
        state.memory.load_elf_section(s);
    }

    // Load in initial program counter
    state.register.write_to_name(
        Register::PC as usize,
        file.ehdr.entry as i32
    );

    state
}

/// Verifies the given ELF file header is compatible with the simulator, and
/// quits if invalid. If this function returns, it can be assumed that the
/// header is good to go!
fn verify_file_header(header: &FileHeader) {
    if header.class != ELFCLASS32 {
        ElfError.exit(Some("Found 64 bit ELF file, expected 32 bit."));
    }
    if header.data != ELFDATA2LSB {
        ElfError.exit(Some("Found Big Endian ELF file, expected Little \
                           Endian."));
    }
    if header.version != EV_CURRENT {
        ElfError.exit(Some("Incompatible ELF file version, expected 1."));
    }
    if header.osabi != ELFOSABI_SYSV {
        ElfError.exit(Some("Incompatible OS ABI in ELF file header, expected \
                           Unix - System V."));
    }
    if header.elftype != ET_EXEC {
        ElfError.exit(Some("Incompatible object file type in ELF file header, \
                           expected EXEC."));
    }
    if header.machine != Machine(0xf3) {
        ElfError.exit(Some("Incompatible ISA in ELF file header, expected \
                           RISC-V."));
    }
}

/// Loose checks to make sure that an _individual_ program header is not
/// something that should break the simulator (e.g. dynamically linked libs),
/// and quits the simulator if invalid. If this function returns, it can be
/// assumed that the header is good to go!
fn verify_prog_header(header: &ProgramHeader) {
    match header.progtype {
        PT_NULL | PT_LOAD | PT_NOTE | PT_PHDR => (),
        _ => ElfError.exit(Some("Elf file contained unsupported program \
                                header type.")),
    }
}

