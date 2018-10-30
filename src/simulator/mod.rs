use std::io;

use util::config::Config;
use util::loader::load_elf;
use self::state::State;

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// Definitions for the execution of every function.
mod instruction;

/// Definitions for the main memory data structure.
pub mod memory;

/// Definitions for the ongoing state of the simulator.
pub mod state;

pub fn run_simulator(config: Config) {
    let memory = load_elf(&config);
    let state  = State {
        register: vec!(0i32, 33),
        memory,
    };

    // Buffer not used, just to pause for user input
    let mut buf = String::new();
    loop {
        println!("Done thing.");
        io::stdin().read_line(&mut buf);
    }
}
