use std::collections::HashMap;

use isa::Word;
use isa::operand::Register;
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

