use std::fmt;

use instruction::Decodable;
use instruction::Format;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// An enum of all the different base opcodes that are provided by `rv32im`.
///
///  - These are always in bits `6-0` of the instruction, and should have
///    `11` in bits `0` and `1`.
///  - These correspond to the assembly op codes for all instruction types that
///    do not have an associated function code.
#[derive(Copy, Clone, PartialEq)]
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

/// An enum of all the different function opcodes that are provided by 
/// `rv32im`. 
///
///  - These correspond to the assembly op codes for all instruction
///    types that have a function code. 
///  - These are not necessarily from one contiguous bit-range within the
///    instruction, as function codes may be spread across multiple parts of 
///    the instruction.
#[derive(Copy, Clone, PartialEq)]
pub enum FunctCode {
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

impl From<FunctCode> for BaseCode {
    /// Finds the associated BaseCode for a given function code.
    fn from(funct_code: FunctCode) -> BaseCode {
        match funct_code {
            FunctCode::JALR   => BaseCode::JALR,
            FunctCode::BEQ    => BaseCode::BRANCH,
            FunctCode::BNE    => BaseCode::BRANCH,
            FunctCode::BLT    => BaseCode::BRANCH,
            FunctCode::BGE    => BaseCode::BRANCH,
            FunctCode::BLTU   => BaseCode::BRANCH,
            FunctCode::BGEU   => BaseCode::BRANCH,
            FunctCode::LB     => BaseCode::LOAD,
            FunctCode::LH     => BaseCode::LOAD,
            FunctCode::LW     => BaseCode::LOAD,
            FunctCode::LBU    => BaseCode::LOAD,
            FunctCode::LHU    => BaseCode::LOAD,
            FunctCode::SB     => BaseCode::STORE,
            FunctCode::SH     => BaseCode::STORE,
            FunctCode::SW     => BaseCode::STORE,
            FunctCode::ADDI   => BaseCode::OPIMM,
            FunctCode::SLTI   => BaseCode::OPIMM,
            FunctCode::SLTIU  => BaseCode::OPIMM,
            FunctCode::XORI   => BaseCode::OPIMM,
            FunctCode::ORI    => BaseCode::OPIMM,
            FunctCode::ANDI   => BaseCode::OPIMM,
            FunctCode::SLLI   => BaseCode::OPIMM,
            FunctCode::SRLI   => BaseCode::OPIMM,
            FunctCode::SRAI   => BaseCode::OPIMM,
            FunctCode::ADD    => BaseCode::OP,
            FunctCode::SUB    => BaseCode::OP,
            FunctCode::SLL    => BaseCode::OP,
            FunctCode::SLT    => BaseCode::OP,
            FunctCode::SLTU   => BaseCode::OP,
            FunctCode::XOR    => BaseCode::OP,
            FunctCode::SRL    => BaseCode::OP,
            FunctCode::SRA    => BaseCode::OP,
            FunctCode::OR     => BaseCode::OP,
            FunctCode::AND    => BaseCode::OP,
            FunctCode::FENCE  => BaseCode::MISCMEM,
            FunctCode::FENCEI => BaseCode::MISCMEM,
            FunctCode::ECALL  => BaseCode::SYSTEM,
            FunctCode::EBREAK => BaseCode::SYSTEM,
            FunctCode::CSRRW  => BaseCode::SYSTEM,
            FunctCode::CSRRS  => BaseCode::SYSTEM,
            FunctCode::CSRRC  => BaseCode::SYSTEM,
            FunctCode::CSRRWI => BaseCode::SYSTEM,
            FunctCode::CSRRSI => BaseCode::SYSTEM,
            FunctCode::CSRRCI => BaseCode::SYSTEM,
            FunctCode::MUL    => BaseCode::OP,
            FunctCode::MULH   => BaseCode::OP,
            FunctCode::MULHSU => BaseCode::OP,
            FunctCode::MULHU  => BaseCode::OP,
            FunctCode::DIV    => BaseCode::OP,
            FunctCode::DIVU   => BaseCode::OP,
            FunctCode::REM    => BaseCode::OP,
            FunctCode::REMU   => BaseCode::OP,
        }
    }
}

impl Decodable for BaseCode {
    fn from_instruction(instruction: u32) -> Option<BaseCode> {
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
    /// Checks if the instruction format of this `BaseCode` has a function code
    /// included within it.
    pub fn has_funct_code(self) -> bool {
        Format::from(self).has_funct_code()
    }

    /// Checks if the instruction format of this `BaseCode` has a `funct7`
    /// section.
    pub fn has_funct7(self) -> bool {
        Format::from(self).has_funct7()
    }
}

///////////////////////////////////////////////////////////////////// FunctCode

impl fmt::Display for FunctCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FunctCode::JALR   => f.pad("JALR"),
            FunctCode::BEQ    => f.pad("BEQ"),
            FunctCode::BNE    => f.pad("BNE"),
            FunctCode::BLT    => f.pad("BLT"),
            FunctCode::BGE    => f.pad("BGE"),
            FunctCode::BLTU   => f.pad("BLTU"),
            FunctCode::BGEU   => f.pad("BGEU"),
            FunctCode::LB     => f.pad("LB"),
            FunctCode::LH     => f.pad("LH"),
            FunctCode::LW     => f.pad("LW"),
            FunctCode::LBU    => f.pad("LBU"),
            FunctCode::LHU    => f.pad("LHU"),
            FunctCode::SB     => f.pad("SB"),
            FunctCode::SH     => f.pad("SH"),
            FunctCode::SW     => f.pad("SW"),
            FunctCode::ADDI   => f.pad("ADDI"),
            FunctCode::SLTI   => f.pad("SLTI"),
            FunctCode::SLTIU  => f.pad("SLTIU"),
            FunctCode::XORI   => f.pad("XORI"),
            FunctCode::ORI    => f.pad("ORI"),
            FunctCode::ANDI   => f.pad("ANDI"),
            FunctCode::SLLI   => f.pad("SLLI"),
            FunctCode::SRLI   => f.pad("SRLI"),
            FunctCode::SRAI   => f.pad("SRAI"),
            FunctCode::ADD    => f.pad("ADD"),
            FunctCode::SUB    => f.pad("SUB"),
            FunctCode::SLL    => f.pad("SLL"),
            FunctCode::SLT    => f.pad("SLT"),
            FunctCode::SLTU   => f.pad("SLTU"),
            FunctCode::XOR    => f.pad("XOR"),
            FunctCode::SRL    => f.pad("SRL"),
            FunctCode::SRA    => f.pad("SRA"),
            FunctCode::OR     => f.pad("OR"),
            FunctCode::AND    => f.pad("AND"),
            FunctCode::FENCE  => f.pad("FENCE"),
            FunctCode::FENCEI => f.pad("FENCEI"),
            FunctCode::ECALL  => f.pad("ECALL"),
            FunctCode::EBREAK => f.pad("EBREAK"),
            FunctCode::CSRRW  => f.pad("CSRRW"),
            FunctCode::CSRRS  => f.pad("CSRRS"),
            FunctCode::CSRRC  => f.pad("CSRRC"),
            FunctCode::CSRRWI => f.pad("CSRRWI"),
            FunctCode::CSRRSI => f.pad("CSRRSI"),
            FunctCode::CSRRCI => f.pad("CSRRCI"),
            FunctCode::MUL    => f.pad("MUL"),
            FunctCode::MULH   => f.pad("MULH"),
            FunctCode::MULHSU => f.pad("MULHSU"),
            FunctCode::MULHU  => f.pad("MULHU"),
            FunctCode::DIV    => f.pad("DIV"),
            FunctCode::DIVU   => f.pad("DIVU"),
            FunctCode::REM    => f.pad("REM"),
            FunctCode::REMU   => f.pad("REMU"),
        }
    }
}

impl Decodable for FunctCode {
    fn from_instruction(instruction: u32) -> Option<FunctCode> {
        // To match Function Code, we first need the base code
        let base_code = match BaseCode::from_instruction(instruction) {
            Some(b) => b,
            None    => return None,
        };
        // Parse out funct3, or return none (funct3 required)
        let funct3 = match base_code.has_funct_code() {
            true  => (instruction >> 12) & 0x7,
            false => return None,
        };
        // Parse out funct7, if required.
        let funct7 = match base_code.has_funct7() {
            true  => instruction >> 25,
            false => 0, // Not required
        };
        // Match on the base code and funct 3, dealing with ambiguities by
        // checking special cases.
        match base_code {
            BaseCode::LOAD => 
                match funct3 {
                    0x0 => Some(FunctCode::LB),
                    0x1 => Some(FunctCode::LH),
                    0x2 => Some(FunctCode::LW),
                    0x4 => Some(FunctCode::LBU),
                    0x5 => Some(FunctCode::LHU),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::MISCMEM => 
                match funct3 {
                    0x0 => Some(FunctCode::FENCE),
                    0x1 => Some(FunctCode::FENCEI),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::OPIMM => 
                match funct3 {
                    0x0 => Some(FunctCode::ADDI),
                    0x2 => Some(FunctCode::SLTI),
                    0x3 => Some(FunctCode::SLTIU),
                    0x4 => Some(FunctCode::XORI),
                    0x6 => Some(FunctCode::ORI),
                    0x7 => Some(FunctCode::ANDI),
                    0x1 => Some(FunctCode::SLLI),
                    0x5 => // Ambiguous Case; Match on func7
                        match funct7 {
                            0x00 => Some(FunctCode::SRLI),
                            0x20 => Some(FunctCode::SRAI),
                            _    => None // Unrecognised funct7
                        },
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::STORE => 
                match funct3 {
                    0x0 => Some(FunctCode::SB),
                    0x1 => Some(FunctCode::SH),
                    0x2 => Some(FunctCode::SW),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::OP => 
                match funct7 {
                    0x00 =>
                        match funct3 {
                            0x0 => Some(FunctCode::ADD),
                            0x1 => Some(FunctCode::SLL),
                            0x2 => Some(FunctCode::SLT),
                            0x3 => Some(FunctCode::SLTU),
                            0x4 => Some(FunctCode::XOR),
                            0x5 => Some(FunctCode::SRL),
                            0x6 => Some(FunctCode::OR),
                            0x7 => Some(FunctCode::AND),
                            _   => None, // Unrecognised funct3
                        },
                    0x20 =>
                        match funct3 {
                            0x0 => Some(FunctCode::SUB),
                            0x5 => Some(FunctCode::SRA),
                            _   => None, // Unrecognised funct3
                        },
                    0x01 =>
                        match funct3 {
                            0x0 => Some(FunctCode::MUL),
                            0x1 => Some(FunctCode::MULH),
                            0x2 => Some(FunctCode::MULHSU),
                            0x3 => Some(FunctCode::MULHU),
                            0x4 => Some(FunctCode::DIV),
                            0x5 => Some(FunctCode::DIVU),
                            0x6 => Some(FunctCode::REM),
                            0x7 => Some(FunctCode::REMU),
                            _   => None // Unrecognised funct3
                        },
                    _ => None // Unrecognised funct7
                    
                },
            BaseCode::BRANCH => 
                match funct3 {
                    0x0 => Some(FunctCode::BEQ),
                    0x1 => Some(FunctCode::BNE),
                    0x4 => Some(FunctCode::BLT),
                    0x5 => Some(FunctCode::BGE),
                    0x6 => Some(FunctCode::BLTU),
                    0x7 => Some(FunctCode::BGEU),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::JALR => 
                match funct3 {
                    0x0 => Some(FunctCode::JALR),
                    _   => None, // Unrecognised funct 3
                },
            BaseCode::SYSTEM => 
                match funct3 {
                    0x0 => // Ambiguous Case (PRIV); Match on funct12
                        match instruction >> 20 {
                            0x0 => Some(FunctCode::ECALL),
                            0x1 => Some(FunctCode::EBREAK),
                            _   => None, // Unrecognised funct12
                        },
                    0x1 => Some(FunctCode::CSRRW),
                    0x2 => Some(FunctCode::CSRRS),
                    0x3 => Some(FunctCode::CSRRC),
                    0x5 => Some(FunctCode::CSRRWI),
                    0x6 => Some(FunctCode::CSRRSI),
                    0x7 => Some(FunctCode::CSRRCI),
                    _   => None, // Unrecognised funct3
                },
            _ => None, // Unrecognised base code
        }
    }
}

