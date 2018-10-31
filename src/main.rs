extern crate byteorder;
extern crate elf;
extern crate tui;
extern crate termion;

use std::env;

use util::config::Config;
use util::exit::Exit;

/// Simulator printout display logic.
mod display;

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
    let mut terminal = display::initialize()
                                .expect("Failed to startup fancy UI.");
    simulator::run_simulator(config);
    println!("Goodbye!\r");
}

