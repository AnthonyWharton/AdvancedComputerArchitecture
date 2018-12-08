use std::fmt;

use super::Format;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// An enum of all the different base opcodes that are provided by `rv32im`.
///
///  - These are always in bits `6-0` of the instruction, and should have
///    `11` in bits `0` and `1`.
///  - These correspond to the assembly op codes for all instruction types that
///    do not have an associated function code.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BaseCode {
    LOAD,
    MISCMEM,
    OPIMM,
    AUIPC,
    STORE,
    OP,
    LUI,
    BRANCH,
    JALR,
    JAL,
    SYSTEM,
}

/// An enum of all the different operations that are provided by `rv32im`.
///
/// These can be parse from a mixture of the `BaseCode` and/or the function
/// code(s) within the instruction. Therefore, these are not necessarily
/// derived from one contiguous bit-range within the instruction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operation {
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    FENCE,
    FENCEI,
    ECALL,
    EBREAK,
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,
}

///////////////////////////////////////////////////////////////////////////////
//// TRAITS

/// Trait for objects that can decode an instruction into an internal
/// representation.
pub trait Decodable {
    /// Decodes a full instruction word, into an internal representation.
    /// Returns None on a failure.
    fn from_instruction(instruction: i32) -> Option<Self> where
        Self: Sized;
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

////////////////////////////////////////////////////////////////////// BaseCode

impl fmt::Display for BaseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseCode::LOAD    => f.pad("LOAD"),
            BaseCode::MISCMEM => f.pad("MISCMEM"),
            BaseCode::OPIMM   => f.pad("OPIMM"),
            BaseCode::AUIPC   => f.pad("AUIPC"),
            BaseCode::STORE   => f.pad("STORE"),
            BaseCode::OP      => f.pad("OP"),
            BaseCode::LUI     => f.pad("LUI"),
            BaseCode::BRANCH  => f.pad("BRANCH"),
            BaseCode::JALR    => f.pad("JALR"),
            BaseCode::JAL     => f.pad("JAL"),
            BaseCode::SYSTEM  => f.pad("SYSTEM"),
        }
    }
}

impl From<Operation> for BaseCode {
    /// Finds the associated BaseCode for a given operation.
    fn from(operation: Operation) -> BaseCode {
        match operation {
            Operation::LUI    => BaseCode::LUI,
            Operation::AUIPC  => BaseCode::AUIPC,
            Operation::JAL    => BaseCode::JAL,
            Operation::JALR   => BaseCode::JALR,
            Operation::BEQ    => BaseCode::BRANCH,
            Operation::BNE    => BaseCode::BRANCH,
            Operation::BLT    => BaseCode::BRANCH,
            Operation::BGE    => BaseCode::BRANCH,
            Operation::BLTU   => BaseCode::BRANCH,
            Operation::BGEU   => BaseCode::BRANCH,
            Operation::LB     => BaseCode::LOAD,
            Operation::LH     => BaseCode::LOAD,
            Operation::LW     => BaseCode::LOAD,
            Operation::LBU    => BaseCode::LOAD,
            Operation::LHU    => BaseCode::LOAD,
            Operation::SB     => BaseCode::STORE,
            Operation::SH     => BaseCode::STORE,
            Operation::SW     => BaseCode::STORE,
            Operation::ADDI   => BaseCode::OPIMM,
            Operation::SLTI   => BaseCode::OPIMM,
            Operation::SLTIU  => BaseCode::OPIMM,
            Operation::XORI   => BaseCode::OPIMM,
            Operation::ORI    => BaseCode::OPIMM,
            Operation::ANDI   => BaseCode::OPIMM,
            Operation::SLLI   => BaseCode::OPIMM,
            Operation::SRLI   => BaseCode::OPIMM,
            Operation::SRAI   => BaseCode::OPIMM,
            Operation::ADD    => BaseCode::OP,
            Operation::SUB    => BaseCode::OP,
            Operation::SLL    => BaseCode::OP,
            Operation::SLT    => BaseCode::OP,
            Operation::SLTU   => BaseCode::OP,
            Operation::XOR    => BaseCode::OP,
            Operation::SRL    => BaseCode::OP,
            Operation::SRA    => BaseCode::OP,
            Operation::OR     => BaseCode::OP,
            Operation::AND    => BaseCode::OP,
            Operation::FENCE  => BaseCode::MISCMEM,
            Operation::FENCEI => BaseCode::MISCMEM,
            Operation::ECALL  => BaseCode::SYSTEM,
            Operation::EBREAK => BaseCode::SYSTEM,
            Operation::CSRRW  => BaseCode::SYSTEM,
            Operation::CSRRS  => BaseCode::SYSTEM,
            Operation::CSRRC  => BaseCode::SYSTEM,
            Operation::CSRRWI => BaseCode::SYSTEM,
            Operation::CSRRSI => BaseCode::SYSTEM,
            Operation::CSRRCI => BaseCode::SYSTEM,
            Operation::MUL    => BaseCode::OP,
            Operation::MULH   => BaseCode::OP,
            Operation::MULHSU => BaseCode::OP,
            Operation::MULHU  => BaseCode::OP,
            Operation::DIV    => BaseCode::OP,
            Operation::DIVU   => BaseCode::OP,
            Operation::REM    => BaseCode::OP,
            Operation::REMU   => BaseCode::OP,
        }
    }
}

impl Decodable for BaseCode {
    fn from_instruction(instruction: i32) -> Option<BaseCode> {
        match instruction & 0x7f {
            0x03 => Some(BaseCode::LOAD),
            0x0f => Some(BaseCode::MISCMEM),
            0x13 => Some(BaseCode::OPIMM),
            0x17 => Some(BaseCode::AUIPC),
            0x23 => Some(BaseCode::STORE),
            0x33 => Some(BaseCode::OP),
            0x37 => Some(BaseCode::LUI),
            0x63 => Some(BaseCode::BRANCH),
            0x67 => Some(BaseCode::JALR),
            0x6F => Some(BaseCode::JAL),
            0x73 => Some(BaseCode::SYSTEM),
            _    => None, // Unrecognised
        }
    }
}

impl BaseCode {
    /// Checks if the instruction format has a destination register encoded
    /// within it, as per the `rv32im` specification.
    pub fn has_rd(self) -> bool {
        match Format::from(self) {
            Format::S |
            Format::B => false,
            _         => true
        }
    }

    /// Checks if the instruction format has a source (1) register encoded
    /// within it, as per the `rv32im` specification.
    pub fn has_rs1(self) -> bool {
        match Format::from(self) {
            Format::U |
            Format::J => false,
            _         => true
        }
    }

    /// Checks if the instruction format has a source (2) register encoded
    /// within it, as per the `rv32im` specification.
    pub fn has_rs2(self) -> bool {
        match Format::from(self) {
            Format::I |
            Format::U |
            Format::J => false,
            _         => true
        }
    }

    /// Checks if the instruction format has a function code included within
    /// it, as per the `rv32im` specification.
    fn has_funct_code(self) -> bool {
        match Format::from(self) {
            Format::U | Format::J => false,
            _                     => true,
        }
    }

    /// Checks if the instruction format **might** have a `funct7` encoded
    /// within it, as per the `rv32im` specification.
    fn may_have_funct7(self) -> bool {
        match Format::from(self) {
            Format::R | Format::I => true,
            _                     => false,
        }
    }
}

///////////////////////////////////////////////////////////////////// FunctCode

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::LUI    => f.pad("lui"),
            Operation::AUIPC  => f.pad("auipc"),
            Operation::JAL    => f.pad("jal"),
            Operation::JALR   => f.pad("jalr"),
            Operation::BEQ    => f.pad("beq"),
            Operation::BNE    => f.pad("bne"),
            Operation::BLT    => f.pad("blt"),
            Operation::BGE    => f.pad("bge"),
            Operation::BLTU   => f.pad("bltu"),
            Operation::BGEU   => f.pad("bgeu"),
            Operation::LB     => f.pad("lb"),
            Operation::LH     => f.pad("lh"),
            Operation::LW     => f.pad("lw"),
            Operation::LBU    => f.pad("lbu"),
            Operation::LHU    => f.pad("lhu"),
            Operation::SB     => f.pad("sb"),
            Operation::SH     => f.pad("sh"),
            Operation::SW     => f.pad("sw"),
            Operation::ADDI   => f.pad("addi"),
            Operation::SLTI   => f.pad("slti"),
            Operation::SLTIU  => f.pad("sltiu"),
            Operation::XORI   => f.pad("xori"),
            Operation::ORI    => f.pad("ori"),
            Operation::ANDI   => f.pad("andi"),
            Operation::SLLI   => f.pad("slli"),
            Operation::SRLI   => f.pad("srli"),
            Operation::SRAI   => f.pad("srai"),
            Operation::ADD    => f.pad("add"),
            Operation::SUB    => f.pad("sub"),
            Operation::SLL    => f.pad("sll"),
            Operation::SLT    => f.pad("slt"),
            Operation::SLTU   => f.pad("sltu"),
            Operation::XOR    => f.pad("xor"),
            Operation::SRL    => f.pad("srl"),
            Operation::SRA    => f.pad("sra"),
            Operation::OR     => f.pad("or"),
            Operation::AND    => f.pad("and"),
            Operation::FENCE  => f.pad("fence"),
            Operation::FENCEI => f.pad("fencei"),
            Operation::ECALL  => f.pad("ecall"),
            Operation::EBREAK => f.pad("ebreak"),
            Operation::CSRRW  => f.pad("csrrw"),
            Operation::CSRRS  => f.pad("csrrs"),
            Operation::CSRRC  => f.pad("csrrc"),
            Operation::CSRRWI => f.pad("csrrwi"),
            Operation::CSRRSI => f.pad("csrrsi"),
            Operation::CSRRCI => f.pad("csrrci"),
            Operation::MUL    => f.pad("mul"),
            Operation::MULH   => f.pad("mulh"),
            Operation::MULHSU => f.pad("mulhsu"),
            Operation::MULHU  => f.pad("mulhu"),
            Operation::DIV    => f.pad("div"),
            Operation::DIVU   => f.pad("divu"),
            Operation::REM    => f.pad("rem"),
            Operation::REMU   => f.pad("remu"),
        }
    }
}

impl Decodable for Operation {
    fn from_instruction(instruction: i32) -> Option<Operation> {
        // To match Function Code, we first need the base code
        let base_code = match BaseCode::from_instruction(instruction) {
            Some(b) => b,
            None    => return None,
        };
        // Parse out funct3, if required
        let funct3 = if base_code.has_funct_code() {
            (instruction >> 12) & 0b111
        } else {
            0
        };
        // Parse out funct7, if required.
        let funct7 = if base_code.may_have_funct7() {
            (instruction >> 25) & 0b111_1111
        } else {
            0 // Not required
        };
        // Match on the base code and funct 3, dealing with ambiguities by
        // checking special cases.
        match base_code {
            BaseCode::LOAD =>
                match funct3 {
                    0x0 => Some(Operation::LB),
                    0x1 => Some(Operation::LH),
                    0x2 => Some(Operation::LW),
                    0x4 => Some(Operation::LBU),
                    0x5 => Some(Operation::LHU),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::MISCMEM =>
                match funct3 {
                    0x0 => Some(Operation::FENCE),
                    0x1 => Some(Operation::FENCEI),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::OPIMM =>
                match funct3 {
                    0x0 => Some(Operation::ADDI),
                    0x2 => Some(Operation::SLTI),
                    0x3 => Some(Operation::SLTIU),
                    0x4 => Some(Operation::XORI),
                    0x6 => Some(Operation::ORI),
                    0x7 => Some(Operation::ANDI),
                    0x1 => Some(Operation::SLLI),
                    0x5 => // Ambiguous Case; Match on func7
                        match funct7 {
                            0x00 => Some(Operation::SRLI),
                            0x20 => Some(Operation::SRAI),
                            _    => None // Unrecognised funct7
                        },
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::AUIPC =>
                Some(Operation::AUIPC),
            BaseCode::STORE =>
                match funct3 {
                    0x0 => Some(Operation::SB),
                    0x1 => Some(Operation::SH),
                    0x2 => Some(Operation::SW),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::OP =>
                match funct7 {
                    0x00 =>
                        match funct3 {
                            0x0 => Some(Operation::ADD),
                            0x1 => Some(Operation::SLL),
                            0x2 => Some(Operation::SLT),
                            0x3 => Some(Operation::SLTU),
                            0x4 => Some(Operation::XOR),
                            0x5 => Some(Operation::SRL),
                            0x6 => Some(Operation::OR),
                            0x7 => Some(Operation::AND),
                            _   => None, // Unrecognised funct3
                        },
                    0x20 =>
                        match funct3 {
                            0x0 => Some(Operation::SUB),
                            0x5 => Some(Operation::SRA),
                            _   => None, // Unrecognised funct3
                        },
                    0x01 =>
                        match funct3 {
                            0x0 => Some(Operation::MUL),
                            0x1 => Some(Operation::MULH),
                            0x2 => Some(Operation::MULHSU),
                            0x3 => Some(Operation::MULHU),
                            0x4 => Some(Operation::DIV),
                            0x5 => Some(Operation::DIVU),
                            0x6 => Some(Operation::REM),
                            0x7 => Some(Operation::REMU),
                            _   => None // Unrecognised funct3
                        },
                    _ => None // Unrecognised funct7

                },
            BaseCode::LUI =>
                Some(Operation::LUI),
            BaseCode::BRANCH =>
                match funct3 {
                    0x0 => Some(Operation::BEQ),
                    0x1 => Some(Operation::BNE),
                    0x4 => Some(Operation::BLT),
                    0x5 => Some(Operation::BGE),
                    0x6 => Some(Operation::BLTU),
                    0x7 => Some(Operation::BGEU),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::JALR =>
                match funct3 {
                    0x0 => Some(Operation::JALR),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::JAL =>
                Some(Operation::JAL),
            BaseCode::SYSTEM =>
                match funct3 {
                    0x0 => // Ambiguous Case (PRIV); Match on funct12
                        match instruction >> 20 {
                            0x0 => Some(Operation::ECALL),
                            0x1 => Some(Operation::EBREAK),
                            _   => None, // Unrecognised funct12
                        },
                    0x1 => Some(Operation::CSRRW),
                    0x2 => Some(Operation::CSRRS),
                    0x3 => Some(Operation::CSRRC),
                    0x5 => Some(Operation::CSRRWI),
                    0x6 => Some(Operation::CSRRSI),
                    0x7 => Some(Operation::CSRRCI),
                    _   => None, // Unrecognised funct3
                },
        }
    }
}

