//! # Project Daybreak
//! Project Daybreak is a superscalar, out of order, `riscv32im` simulator.
//! It was primarily developed for a piece of coursework whilst studying
//! _Advanced Computer Architecture_ in the Department of Computer Science at
//! the University of Bristol.
//!
//! ![Project Daybreak Simulator Diagram](https://github.com/AnthonyWharton/AdvancedComputerArchitecture/raw/master/resources/diagram.png)

use crate::io::IoThread;
use crate::util::config::Config;

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
    let config = Config::create_from_args();
    let io = IoThread::new();
    simulator::run_simulator(io, &config);
    println!("Goodbye!\r");
}
