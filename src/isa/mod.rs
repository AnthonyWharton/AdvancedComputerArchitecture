use std::fmt::{Display, Formatter, Result};

use self::op_code::{BaseCode, Decodable, Operation};
use self::operand::{extract_immediate, Register, RegisterOperand};

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// All things related to a `rv32im` opcodes.
pub mod op_code;

/// All things related to a `rv32im` operand, i.e. the registers or immediate.
pub mod operand;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// An enum of all the different Instruction formats that are provided by
/// `rv32im`.
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// Struct to encapsulate a decoded instruction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Instruction {
    pub op: Operation,
    pub rd: Option<Register>,
    pub rs1: Option<Register>,
    pub rs2: Option<Register>,
    pub imm: Option<i32>,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

//////////////////////////////////////////////////////////////////////// Format

impl Display for Format {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
    #[rustfmt::skip]
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

impl From<op_code::Operation> for Format {
    /// Provides an Instruction Format given the `Operation` of an instruction.
    fn from(op: Operation) -> Format {
        Format::from(BaseCode::from(op))
    }
}

/////////////////////////////////////////////////////////////////// Instruction

impl Default for Instruction {
    /// Returns a `nop`.
    fn default() -> Instruction {
        Instruction {
            op: Operation::ADDI,
            rd: Some(Register::X0),
            rs1: Some(Register::X0),
            rs2: None,
            imm: Some(0),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.op)?;
        if self.rd.is_some() {
            write!(f, " {:#}", self.rd.as_ref().unwrap())?
        };
        if self.rs1.is_some() {
            write!(f, " {:#}", self.rs1.as_ref().unwrap())?
        };
        if self.rs2.is_some() {
            write!(f, " {:#}", self.rs2.as_ref().unwrap())?
        };
        if self.imm.is_some() {
            write!(f, " {}", self.imm.unwrap())?
        };
        Ok(())
    }
}

impl Instruction {
    /// Decodes a RISC V binary instruction word from the `rv32im`
    /// specification. Returns None if there instruction failed to decode.
    pub fn decode(instruction: i32) -> Option<Instruction> {
        Some(Instruction {
            op: match Operation::from_instruction(instruction) {
                Some(o) => o,
                None => return None,
            },
            rd: Register::extract_register(&RegisterOperand::RD, instruction),
            rs1: Register::extract_register(&RegisterOperand::RS1, instruction),
            rs2: Register::extract_register(&RegisterOperand::RS2, instruction),
            imm: extract_immediate(instruction),
        })
    }
}
