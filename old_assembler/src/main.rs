extern crate definitions;

use std::env;
use config::Config;

pub mod config;
mod io;


fn main() {
    let config = Config::new(env::args()).unwrap();
    println!("Config: {:?}", config);

    let lines = io::read_assembly_lines(&config.assembly_file);
    lines.iter().for_each(|line| println!(" | {}", line));

    io::write_binary(&config, &std::collections::VecDeque::default());
}

pub fn help() {
    println!("Usage: assembler ASSEMBLY [-o OUTPUT]");
    println!("");
    println!("Creates simulator readable binary from assembly code.")
}
