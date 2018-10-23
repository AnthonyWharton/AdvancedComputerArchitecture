use elf::File;
use elf::ParseError;
use elf::types::*;

use util::config::Config;
use util::exit::Exit::FileLoadError;

///////////////////////////////////////////////////////////////////////////////
//// TYPES

/// Type alias for the data structure that holds main memory
type Memory = Box<Vec<u8>>;

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

    println!("ELF_FILE_HEADER:\n{:#?}\n", file.ehdr);
    verify_header(&file.ehdr);
    println!("PROGRAM_HEADERS:\n{:#?}\n", file.phdrs);

    println!("SECTION_HEADERS:\n");
    for s in file.sections.iter() {
        println!("{:#?}", s.shdr);
    }

    let mut mem: Memory = Box::new(vec!(0; 12));

    mem
}

/// Verifies the given ELF file header, and quits the simulator if invalid. If
/// this function returns, it can be assumed that the header is good to go!
fn verify_header(header: &FileHeader) {
    if header.class != ELFCLASS32 { 
        FileLoadError.exit(Some("Found 64 bit ELF file, expected 32 bit."));
    }
    if header.data != ELFDATA2LSB {
        FileLoadError.exit(Some("Found Big Endian ELF file, expected Little Endian."));
    }
    if header.version != EV_CURRENT {
        FileLoadError.exit(Some("Incompatible ELF file version, expected 1.")); 
    }
    if header.osabi != ELFOSABI_SYSV {
        FileLoadError.exit(Some("Incompatible OS ABI in ELF file header, expected Unix - System V."));
    }
    if header.elftype != ET_EXEC { 
        FileLoadError.exit(Some("Incompatible object file type in ELF file header, expected EXEC."));
    }
    if header.machine != Machine(0xf3) {
        FileLoadError.exit(Some("Incompatible ISA in ELF file header, expected RISC-V."));
    }
}

