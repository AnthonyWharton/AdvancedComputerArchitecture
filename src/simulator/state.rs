use std::default::Default;

use crate::isa::operand::Register;

use super::branch::BranchPredictor;
use super::execute::ExecuteUnit;
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
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl State {
    /// Flushes the entire pipeline, restarting from the given Program Counter.
    pub fn flush_pipeline(&mut self, actual_pc: usize) {
        self.branch_predictor.force_update(actual_pc);
        self.register.flush();
        self.latch_fetch.data = None;
        self.resv_station.flush();
        self.reorder_buffer.flush();
        for eu in self.execute_units.iter_mut() {
            eu.flush();
        }
    }
}

impl Default for State {
    fn default() -> State {
        let mut regs = RegisterFile::new(64);
        regs.write_to_name(Register::X2 as usize, 128);
        regs.write_to_name(Register::X8 as usize, 128);
        State {
            stats: Stats::default(),
            memory: Memory::create_empty(INIT_MEMORY_SIZE),
            register: regs,
            branch_predictor: BranchPredictor::new(0),
            latch_fetch: LatchFetch::default(),
            resv_station: ResvStation::new(16),
            reorder_buffer: ReorderBuffer::new(32),
            execute_units: Vec::new(), // TODO address lack of execute units
        }
    }
}
