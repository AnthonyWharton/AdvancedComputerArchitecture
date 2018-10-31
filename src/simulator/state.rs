use std::default::Default;

use isa::Word;
use util::loader::INIT_MEMORY_SIZE;
use super::memory::Memory;

///////////////////////////////////////////////////////////////////////////////
//// TYPES

pub type RegisterFile = Vec<Word>;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

pub struct State {
    pub register: RegisterFile,
    pub memory:   Memory,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl Default for State {
    fn default() -> State {
        State {
            register: vec!(0 as Word; 33),
            memory: Memory::create_empty(INIT_MEMORY_SIZE),
        }
    }
}

