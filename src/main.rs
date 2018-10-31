extern crate byteorder;
extern crate elf;
extern crate tui;
extern crate termion;

use std::env;

use util::config::Config;
use util::exit::Exit;

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// All input/output logic, including interfacing with the IO thread.
mod io;

/// Everything to do isa instructions; definitions and binary logic/parsing etc.
mod isa;

/// Everything to do with the simulator; logic and virtual components etc.
mod simulator;

/// Miscellaneous Utilities and Helpers
mod util;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Main entry point, not much else to say.
fn main() {
    let config = match Config::new(env::args()) {
        Ok(c)  => c,
        Err(e) => Exit::ArgumentError.exit(Some(e)),
    };
    simulator::run_simulator(config);
    println!("Goodbye!\r");
}

