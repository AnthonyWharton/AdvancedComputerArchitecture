use std::fmt;

/// All things related to a `rv32im` opcodes.
pub mod op_code;

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
//// TRAITS

/// Trait for objects that can decode an instruction into an internal
/// representation.
pub trait Decodable {
    /// Decodes a full instruction word, into an internal representation.
    /// Returns None on a failure.
    fn from_instruction(instruction: u32) -> Option<Self> where
        Self: Sized;
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
    fn from(code: op_code::BaseCode) -> Format {
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

impl Format {
    /// Checks if the instruction format has a function code included within
    /// it, as per the `rv32im` specification.
    pub fn has_funct_code(&self) -> bool {
        match self {
            Format::U | Format::J => false,
            _                     => true,
        }
    }

    /// Checks if the instruction format has a `funct7` section.
    pub fn has_funct7(&self) -> bool {
        match self {
            Format::R => true,
            _         => false,
        }
    }
}
