extern crate byteorder;
extern crate elf;

use std::env;

use isa::Instruction;
use util::config::Config;
use util::exit::Exit;

/// Everything to do isa instructions; definitions and binary logic/parsing etc.
mod isa;

/// Everything to do with the simulator; logic and virtual components etc.
mod simulator;

/// Miscellaneous Utilities and Helpers
mod util;

/// Main entry point, not much else to say.
fn main() {
    let config = match Config::new(env::args()) {
        Ok(c)  => c,
        Err(e) => Exit::ArgumentError.exit(Some(e)),
    };

    println!("Read Config: {:?}", config);
    let _memory = util::loader::load_elf(&config);
    let test   = Instruction::decode(0x02010413);
    println!("Test Instruction: {}", test.unwrap());
}

