use clap::{App, Arg};

use crate::simulator::branch::BranchPredictorMode;

/// Encapsulates the settings for the simulator to run with.
#[derive(Debug)]
pub struct Config {
    /// The path of the elf-file to run in the simulator.
    pub elf_file: String,
    /// The _n-way-ness_ of the _fetch_, _decode_ and _commit_ stages in the
    /// processor pipeline.
    pub n_way: usize,
    /// The amount of instructions that can be issued every cycle, and
    /// subsequently the number that can be commited. If this is 0, it will be
    /// assumed to be the number of execute units in the simulator.
    pub issue_limit: usize,
    /// The number of Arithmetic Logic Units the simulator should have.
    pub alu_units: usize,
    /// The number of Branch Logic Units the simulator should have.
    pub blu_units: usize,
    /// The number of Memory Control Units the simulator should have.
    pub mcu_units: usize,
    /// The number of entries in the reservation station.
    pub rsv_size: usize,
    /// The number of entries in the reorder buffer.
    pub rob_size: usize,
    /// Whether or not branch prediction is enabled.
    pub branch_prediction: BranchPredictorMode,
    /// Whether or not a return address stack is being used.
    pub return_address_stack: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            elf_file: String::from(""),
            n_way: 1,
            issue_limit: 1,
            alu_units: 1,
            blu_units: 1,
            mcu_units: 1,
            rsv_size: 16,
            rob_size: 32,
            branch_prediction: BranchPredictorMode::default(),
            return_address_stack: false,
        }
    }
}

impl Config {
    /// Generates a new Config for the assembler program given the arguments
    pub fn create_from_args() -> Config {
        let matches = App::new("Project Daybreak")
                          .version("0.1.0")
                          .author("Anthony W. <a.wharton.2015@bristol.ac.uk>")
                          .about("A superscalar, out of order, riscv32im simulator.")
                          .max_term_width(100)
                          .arg(Arg::with_name("elf-file")
                               .takes_value(true)
                               .value_name("FILE")
                               .required(true)
                               .help("Specifies a path to elf file to execute in the simulator."))
                          .arg(Arg::with_name("n-way")
                               .short("n")
                               .long("n-way")
                               .takes_value(true)
                               .value_name("N")
                               .default_value("1")
                               .validator(|s| match s.parse::<usize>() {
                                   Ok(_) => Ok(()),
                                   Err(_) => Err(String::from("Not a valid number!"))
                               })
                               .required(false)
                               .help("Sets the 'n-way-ness' of the fetch, decode and commit stages."))
                          .arg(Arg::with_name("issue-limit")
                               .short("i")
                               .long("issue-limit")
                               .takes_value(true)
                               .value_name("N")
                               .default_value("1")
                               .validator(|s| match s.parse::<usize>() {
                                   Ok(_) => Ok(()),
                                   Err(_) => Err(String::from("Not a valid number!"))
                               })
                               .required(false)
                               .help("Sets a limit to the number of instructions issued and committed per cycle. Setting this to 0 is interpreted as the number of execute units."))
                          .arg(Arg::with_name("alu-units")
                               .long("alu")
                               .takes_value(true)
                               .value_name("N")
                               .default_value("1")
                               .validator(|s| match s.parse::<usize>() {
                                   Ok(_) => Ok(()),
                                   Err(_) => Err(String::from("Not a valid number!"))
                               })
                               .required(false)
                               .help("Sets the number of Arithmetic Logic Units."))
                          .arg(Arg::with_name("blu-units")
                               .long("blu")
                               .takes_value(true)
                               .value_name("N")
                               .default_value("1")
                               .validator(|s| match s.parse::<usize>() {
                                   Ok(_) => Ok(()),
                                   Err(_) => Err(String::from("Not a valid number!"))
                               })
                               .required(false)
                               .help("Sets the number of Branch Logic Units."))
                          .arg(Arg::with_name("mcu-units")
                               .long("mcu")
                               .takes_value(true)
                               .value_name("N")
                               .default_value("1")
                               .validator(|s| match s.parse::<usize>() {
                                   Ok(_) => Ok(()),
                                   Err(_) => Err(String::from("Not a valid number!"))
                               })
                               .required(false)
                               .help("Sets the number of Memory Control Units."))
                          .arg(Arg::with_name("rsv-size")
                               .long("rsv")
                               .takes_value(true)
                               .value_name("N")
                               .default_value("16")
                               .validator(|s| match s.parse::<usize>() {
                                   Ok(_) => Ok(()),
                                   Err(_) => Err(String::from("Not a valid number!"))
                               })
                               .required(false)
                               .help("Sets the number of entries in the reservation station."))
                          .arg(Arg::with_name("rob-size")
                               .long("rob")
                               .takes_value(true)
                               .value_name("N")
                               .default_value("32")
                               .validator(|s| match s.parse::<usize>() {
                                   Ok(_) => Ok(()),
                                   Err(_) => Err(String::from("Not a valid number!"))
                               })
                               .required(false)
                               .help("Sets the number of entries in the reorder buffer."))
                          .arg(Arg::with_name("branch-prediction")
                               .short("b")
                               .long("branch-prediction")
                               .takes_value(true)
                               .possible_values(&["off", "onebit", "twobit", "twolevel"])
                               .default_value("twobit")
                               .case_insensitive(true)
                               .required(false)
                               .help("Sets the branch prediction mode."))
                          .arg(Arg::with_name("return-stack")
                               .short("r")
                               .long("return-stack")
                               .required(false)
                               .requires("branch-prediction")
                               .help("Enables the Return Address Stack."))
                          .get_matches();

        let mut config = Config::default();
        config.elf_file = String::from(matches.value_of("elf-file").unwrap());
        if let Some(s) = matches.value_of("n-way") {
            config.n_way = s.parse::<usize>().unwrap();
        }
        if let Some(s) = matches.value_of("issue-limit") {
            config.issue_limit= s.parse::<usize>().unwrap();
        }
        if let Some(s) = matches.value_of("alu-units") {
            config.alu_units = s.parse::<usize>().unwrap();
        }
        if let Some(s) = matches.value_of("blu-units") {
            config.blu_units = s.parse::<usize>().unwrap();
        }
        if let Some(s) = matches.value_of("mcu-units") {
            config.mcu_units = s.parse::<usize>().unwrap();
        }
        if let Some(s) = matches.value_of("rsv-size") {
            config.rsv_size = s.parse::<usize>().unwrap();
        }
        if let Some(s) = matches.value_of("rob-size") {
            config.rob_size = s.parse::<usize>().unwrap();
        }
        if let Some(s) = matches.value_of("branch-prediction") {
            match s.to_lowercase().as_str() {
                "off" => config.branch_prediction = BranchPredictorMode::Off,
                "onebit" => config.branch_prediction = BranchPredictorMode::OneBit,
                "twobit" => config.branch_prediction = BranchPredictorMode::TwoBit,
                "twolevel" => config.branch_prediction = BranchPredictorMode::TwoLevel,
                _ => (),
            }
        }
        if matches.is_present("return-stack") {
            config.return_address_stack = true;
        }

        config
    }
}
