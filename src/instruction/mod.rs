/// All things related to a `rv32im` opcodes.
pub mod op_code;

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
//// Implementations

//////////////////////////////////////////////////////////////////////// Format

impl Format {
    /// Checks if the instruction format has a function code included within 
    /// it, as per the `rv32im` specification.
    pub fn has_funct_code(self) -> bool {
        match self {
            Format::U | Format::J => false,
            _                     => true,
        }
    }

    /// Checks if the instruction format has a `funct7` section.
    pub fn has_funct7(self) -> bool {
        match self {
            Format::R => true,
            _         => false,
        }
    }
}
