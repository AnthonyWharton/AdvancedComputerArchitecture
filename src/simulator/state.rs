use std::default::Default;

use isa::Instruction;
use isa::operand::Register;
use super::memory::{Access, INIT_MEMORY_SIZE, Memory};

///////////////////////////////////////////////////////////////////////////////
//// TYPES

/// The entire physical register file.
pub type RegisterFile = [i32; 33];

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Current state of the simulator at any given moment.
#[derive(Clone)]
pub struct State {
    pub stats: Stats,
    pub memory: Memory,
    pub register: RegisterFile,
    pub l_fetch: Option<Access<i32>>,
    pub l_decode: Option<Instruction>,
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

impl Default for State {
    fn default() -> State {
        let mut regs = [0i32; 33];
        regs[Register::X2 as usize] = 128;
        regs[Register::X8 as usize] = 128;
        State {
            stats: Stats::default(),
            memory: Memory::create_empty(INIT_MEMORY_SIZE),
            register: regs,
            l_fetch: None,
            l_decode: None,
        }
    }
}

