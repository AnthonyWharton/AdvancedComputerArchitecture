use std::env;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub elf_file: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            elf_file: String::from(""),
        }
    }
}

impl Config {

    /// Generates a new Config for the assembler program given the arguments
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Consume first argument - this is the executable

        let mut elf_file: Option<String> = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => help(),

                _ => {
                    if elf_file == None {
                        elf_file = Some(arg);
                    } else {
                        return Err("More than one binary elf file specified");
                    }
                },
            }
        }

        if elf_file == None {
            return Err("No binary elf file specified!")
        }

        Ok(Config {
            elf_file: elf_file.unwrap(),
        })
    }
}

/// Prints out a usage/help printout for this simulator.
fn help() {
    println!("Usage: simulator ELF_EXECUTABLE [-h|--help]");
    println!("");
    println!("Simulates a RISC V rv32im binary ELF32 file.");
    process::exit(1);
}

