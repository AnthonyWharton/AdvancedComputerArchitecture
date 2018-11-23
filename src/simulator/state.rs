use std::default::Default;

use isa::Instruction;
use isa::operand::Register;
use super::memory::{INIT_MEMORY_SIZE, Memory};

///////////////////////////////////////////////////////////////////////////////
//// TYPES

pub type RegisterFile = [i32; 33];

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

#[derive(Clone)]
pub struct State {
    pub memory: Memory,
    pub register:  RegisterFile,
    pub lp_fetch:  i32,
    pub lp_decode: Instruction,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl Default for State {
    fn default() -> State {
        let mut regs = [0i32; 33];
        regs[Register::X2 as usize] = 128;
        regs[Register::X8 as usize] = 128;
        State {
            memory: Memory::create_empty(INIT_MEMORY_SIZE),
            register: regs,
            lp_fetch: 0,
            lp_decode: Instruction::default(),
        }
    }
}

