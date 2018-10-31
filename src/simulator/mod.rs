use std::io;

use termion::event::Key;

use display::input::{InputHandler, EXIT_KEYS};
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

    let input = InputHandler::new();

    // Buffer not used, just to pause for user input
    let mut buf = String::new();
    loop {
        match input.next() {
            Ok(key) => match key {
                k if EXIT_KEYS.contains(&k) => break,
                _ => println!("Done thing.\r"),
            }
            Err(_)  => {}
        }
    }
}
