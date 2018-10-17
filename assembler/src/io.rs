use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

use config::Config;

/// # read_assembly_lines
/// Reads all the lines in the given assembly file path into a vector of 
/// strings.
pub fn read_assembly_lines(assembly_file: &String) -> Vec<String> {
    let file = File::open(&assembly_file)
                    .expect("Unable to open assembly file.");
    let mut lines  = BufReader::new(file).lines();
    let mut result = Vec::new();

    while let Some(line) = lines.next() {
        match line {
            Ok(l)  => result.push(l),
            Err(e) => println!("Error whilst reading assembly file, {:?}", e.kind())
        }
    }

    result
}

/// # write_binary
/// Writes the given binary output to a file. Filename is provided by the config
/// or generated if not provided.
pub fn write_binary(config: &Config, _output: &VecDeque<i64>) {
    let output_file = prepare_output_file(&config.output_file);
    println!("Will someday output to {}", output_file);
}

/// # prepare_filename
/// Returns a Path that is safe to output to, even if an overwrite is required.
///
/// If the optionally provided path String is provided, will check if the file
/// exists, or verify that the user has permitted overwriting of said file.
///
/// If the optionally provided path string is not provided, will generate an
/// output filename.
fn prepare_output_file(path: &Option<String>) -> String {

    let path = match path {
        Some(p) => Path::new(p.as_str()),
        None    => Path::new("out.bin"),
    };

    if !path.exists() {
        return String::from(path.to_str().unwrap());
    }

    print!("Output file {} exists, do you want to overwrite it? [y|n] ", 
           path.to_str().unwrap());
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() == "y" {
        return String::from(path.to_str().unwrap());
    } else {
        panic!("Aborted, not allowed to overrite output file {}", 
               path.to_str().unwrap());
    }
}
