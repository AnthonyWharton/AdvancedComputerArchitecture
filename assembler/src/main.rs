extern crate definitions;

use std::env;

pub mod config;
use config::Config;

fn main() {
    let config = Config::new(env::args());
    if let Err(error) = config {
        println!("Argument Error: {}", error);
        std::process::exit(1);
    }
    println!("Config: {:?}", config);
}

pub fn help() {
    println!("Usage: assembler ASSEMBLY [-o OUTPUT]");
    println!("");
    println!("Creates simulator readable binary from assembly code.")
}
