extern crate elf;

use elf::File;

/// Main entry point, not much else to say.
fn main() { 
    let file = File::new();
    println!("File: {}", file.ehdr.machine);
}

