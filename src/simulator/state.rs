use std::default::Default;

use crate::isa::operand::Register;
use crate::util::config::Config;
use crate::util::loader::load_elf;

use super::branch::BranchPredictor;
use super::execute::{ExecuteUnit, UnitType};
use super::fetch::LatchFetch;
use super::memory::{Memory, INIT_MEMORY_SIZE};
use super::register::RegisterFile;
use super::reorder::ReorderBuffer;
use super::reservation::ResvStation;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Current state of the simulator at any given moment.
#[derive(Clone)]
pub struct State {
    /// Statistics collected over the simulator's lifetime.
    pub stats: Stats,
    /// Used to display debug prints in the debug thread.
    pub debug_msg: Vec<String>,
    /// The _n-way-ness_ of the superscalar _fetch_, _decode_ and _commit_
    /// stages in the pipeline. (Note: _dispatch_ and _execute_ are always
    /// `exec_units.len()`-way superscalar.
    pub n_way: usize,
    /// The virtual memory module, holding data and instructions in the
    /// simulated machine.
    pub memory: Memory,
    /// The virtual register file, holding both architectural and physical
    /// registers for the simulated machine.
    pub register: RegisterFile,
    /// The virtual branch predict unit, that is used to select the instruction
    /// that is loaded in the _fetch_ stage.
    pub branch_predictor: BranchPredictor,
    /// The virtual latch after the fetch unit, holding the data that is
    /// fetched after the _fetch_ stage in the pipeline.
    pub latch_fetch: LatchFetch,
    /// The virtual reservation station, that holding instructions pending
    /// execution.
    pub resv_station: ResvStation,
    /// The virtual reorder buffer, holding the pending results ready for
    /// in-order _commitment_ at the writeback stage.
    pub reorder_buffer: ReorderBuffer,
    /// The virtual execute units, used to execute instructions out of order in
    /// the _execute_ stage.
    pub execute_units: Vec<Box<ExecuteUnit>>,
}

/// Container for simulation statistics.
#[derive(Clone, Default)]
pub struct Stats {
    /// The number of cycles that have passed.
    pub cycles: u64,
    /// The number of successfully executed instructions.
    pub executed: u64,
    /// The number of pipeline stalls/bubbles that have occured.
    pub stalls: u64,
    /// The number of branch predictions that were successful.
    pub bp_success: u64,
    /// The number of branch predictions that failed.
    pub bp_failure: u64,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl State {
    /// Creates a new state according to the given config
    pub fn new(config: &Config) -> State {
        // Create register file
        let mut register = RegisterFile::default();
        // Initialise return address to -1 (for detecting exit)
        register[Register::X1].data = -1;
        // Initialise stack pointer to the end of memory.
        register[Register::X2].data = INIT_MEMORY_SIZE as i32 - 4;
        register[Register::X8].data = INIT_MEMORY_SIZE as i32 - 4;

        // Create execution unit(s)
        let execute_units = vec![
            Box::new(ExecuteUnit::new(UnitType::MCU, 1)),
            Box::new(ExecuteUnit::new(UnitType::ALU, 1)),
            Box::new(ExecuteUnit::new(UnitType::ALU, 1)),
            Box::new(ExecuteUnit::new(UnitType::ALU, 1)),
            Box::new(ExecuteUnit::new(UnitType::BLU, 1)),
        ];

        // Create state
        let mut state = State {
            stats: Stats::default(),
            debug_msg: Vec::new(),
            n_way: 4,
            memory: Memory::create_empty(INIT_MEMORY_SIZE),
            register,
            branch_predictor: BranchPredictor::new(0),
            latch_fetch: LatchFetch::default(),
            resv_station: ResvStation::new(16),
            reorder_buffer: ReorderBuffer::new(32),
            execute_units,
        };

        // Load ELF file into the new state
        load_elf(&mut state, &config);

        state
    }

    /// Flushes the entire pipeline, restarting from the given Program Counter.
    pub fn flush_pipeline(&mut self, actual_pc: usize) {
        self.stats.bp_failure += 1;
        self.register.flush();
        self.branch_predictor.force_update(actual_pc);
        self.latch_fetch.data = vec![];
        self.resv_station.flush();
        self.reorder_buffer.flush();
        for eu in self.execute_units.iter_mut() {
            eu.flush();
        }
    }
}

impl Default for State {
    fn default() -> State {
        let mut register = RegisterFile::default();
        // Initialise return address to -1 (for detecting exit)
        register[Register::X1].data = -1;
        // Initialise stack pointer to the end of memory.
        register[Register::X2].data = INIT_MEMORY_SIZE as i32 - 4;
        register[Register::X8].data = INIT_MEMORY_SIZE as i32 - 4;
        State {
            stats: Stats::default(),
            debug_msg: Vec::new(),
            n_way: 1,
            memory: Memory::create_empty(INIT_MEMORY_SIZE),
            register,
            branch_predictor: BranchPredictor::new(0),
            latch_fetch: LatchFetch::default(),
            resv_station: ResvStation::new(16),
            reorder_buffer: ReorderBuffer::new(32),
            execute_units: Vec::new(),
        }
    }
}
