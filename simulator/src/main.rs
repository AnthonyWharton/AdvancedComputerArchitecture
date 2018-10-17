extern crate definitions;
extern crate elf;

use elf::File;

fn main() { 
    let file = File::new();
    println!("File: {}", file.ehdr.machine);
}

