use std::fmt;
use std::fmt::Display;

use elf::File;
use elf::ParseError;

use util::config::Config;
use util::exit::Exit::FileLoadError;

///////////////////////////////////////////////////////////////////////////////
//// TYPES

/// Type alias for the data structure that holds main memory
type Memory = [u32; 1000000];

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

    println!("MAIN_HEADER: {}", file.ehdr);
    println!("PROG_HEADERS: {:?}", file.phdrs);
    println!("SECT_HEADERS: {:?}", file.sections);
    let mut mem: Memory = [0; 1000000];
    mem[0] = 0x10;
    mem[1] = 0x20;
    mem[2] = 0x30;
    println!("Vec Len {} - {:#08x?}", mem.len(), &mem[0..10]);
    mem
}

