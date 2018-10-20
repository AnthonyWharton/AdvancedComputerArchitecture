extern crate elf;

use elf::File;

/// Everything to do with simulator binary instructions; definitions, 
/// logic, parsing etc. 
mod instruction;

/// Main entry point, not much else to say.
fn main() { 
    let file = File::new();
    println!("File: {}", file.ehdr.machine);
}

