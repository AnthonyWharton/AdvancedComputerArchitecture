extern crate elf;

use std::env;

use util::config::Config;
use util::exit::Exit;

/// Everything to do with simulator binary instructions; definitions,
/// logic, parsing etc.
mod instruction;

/// Miscellaneous Utilities and Helpers
mod util;

/// Main entry point, not much else to say.
fn main() {
    let config = match Config::new(env::args()) {
        Ok(c)  => c,
        Err(e) => Exit::ArgumentError.exit(Some(e)),
    };


    println!("Read Config: {:?}", config);
}

