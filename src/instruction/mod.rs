use std::fmt;

use memory::Word;
use self::op_code::{BaseCode, Operation};
use self::operand::Register;

/// All things related to a `rv32im` opcodes.
pub mod op_code;

/// All things related to a `rv32im` operand, i.e. the registers or immediate.
pub mod operand;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// An enum of all the different Instruction formats that are provided by
/// `rv32im`.
#[derive(Copy, Clone, PartialEq)]
pub enum Format {
    R,
    I,
    S,
    B,
    U,
    J,
}

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

pub struct Instruction {
    op:  Operation,
    rd:  Option<Register>,
    rs1: Option<Register>,
    rs2: Option<Register>,
    imm: Option<Word>,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

//////////////////////////////////////////////////////////////////////// Format

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::R => f.pad("R"),
            Format::I => f.pad("I"),
            Format::S => f.pad("S"),
            Format::B => f.pad("B"),
            Format::U => f.pad("U"),
            Format::J => f.pad("J"),
        }
    }
}

impl From<op_code::BaseCode> for Format {
    /// Provides an Instruction Format given the `BaseCode` of an instruction.
    fn from(code: BaseCode) -> Format {
        match code {
            op_code::BaseCode::OP      => Format::R,
            op_code::BaseCode::JALR    |
            op_code::BaseCode::LOAD    |
            op_code::BaseCode::OPIMM   |
            op_code::BaseCode::MISCMEM |
            op_code::BaseCode::SYSTEM  => Format::I,
            op_code::BaseCode::STORE   => Format::S,
            op_code::BaseCode::BRANCH  => Format::B,
            op_code::BaseCode::LUI     |
            op_code::BaseCode::AUIPC   => Format::U,
            op_code::BaseCode::JAL     => Format::J,
        }
    }
}

/////////////////////////////////////////////////////////////////// Instruction

