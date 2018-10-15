use std::env;

#[derive(Debug)]
pub struct Config {
    assembly_file: String,
    output_file:   Option<String>,
}

impl Config {

    /// Generates a new Config for the assembler program given the arguments
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Consume first argument - this is the executable

        let mut assembly_file: Option<String> = None;
        let mut output_file:   Option<String> = None; 

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => super::help(),
            
                "-o" | "--out"  => {
                    if output_file == None {
                        if let Some(output) = args.next() {
                            output_file = Some(output);
                        } else {
                            return Err("No output file specified with output flag");
                        }
                    } else {
                        return Err("More than one output file specified");
                    }
                },
                
                _ => {
                    if assembly_file == None {
                        assembly_file = Some(arg);
                    } else {
                        return Err("More than one assembly file specified");
                    }
                },
            }
        }

        if assembly_file == None {
            return Err("No assembly file specified!")
        }

        Ok(Config {
            assembly_file: assembly_file.unwrap(),
            output_file,
        })
    }
}
