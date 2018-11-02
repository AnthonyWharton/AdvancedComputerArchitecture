use std::default::Default;

use isa::operand::Register;

///////////////////////////////////////////////////////////////////////////////
//// TYPES

pub type RegisterFile = [i32; 33];

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

#[derive(Copy, Clone)]
pub struct State {
    pub register: RegisterFile,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl Default for State {
    fn default() -> State {
        let mut regs = [0i32; 33];
        regs[Register::X2 as usize] = 128;
        regs[Register::X8 as usize] = 128;
        State {
            register: regs,
        }
    }
}

