extern crate byteorder;
extern crate elf;
extern crate either;

use std::env;

use io::IoThread;
use util::config::Config;
use util::exit::Exit;

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// Miscellaneous Utilities and Helpers.
#[macro_use]
mod util;

/// All input/output logic, including interfacing with the IO thread.
mod io;

/// Definitions for the `riscv32im` ISA, and logic for decoding.
mod isa;

/// All of the simulator's components, logic and state.
mod simulator;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Main entry point, not much else to say.
fn main() {
    util::panic::set_panic_hook();
    let io = IoThread::new();
    let config = match Config::new(env::args()) {
        Ok(c)  => c,
        Err(e) => Exit::ArgumentError.exit(Some(e)),
    };
    simulator::run_simulator(io, config);
    println!("Goodbye!\r");
}

