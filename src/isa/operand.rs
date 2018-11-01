use std::fmt;

use isa::{Format, Word};
use isa::op_code::{BaseCode, Decodable};

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

const REG_OP_MASK: Word = 0b11111;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// The ID's of all the registers that are potentially encoded within a
/// `rv32im` specification instruction.
pub enum RegisterOperand {
    RD,
    RS1,
    RS2,
}

/// The ID's of all the registers that are user accesible by executing
/// programs in the `rv32im` specification.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Register {
    X0  =  0,
    X1  =  1,
    X2  =  2,
    X3  =  3,
    X4  =  4,
    X5  =  5,
    X6  =  6,
    X7  =  7,
    X8  =  8,
    X9  =  9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    X16 = 16,
    X17 = 17,
    X18 = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    X29 = 29,
    X30 = 30,
    X31 = 31,
    PC  = 32,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::X0  =>
                if f.alternate() { f.pad("zero") } else { f.pad("x0")  },
            Register::X1  =>
                if f.alternate() { f.pad("ra")   } else { f.pad("x1")  },
            Register::X2  =>
                if f.alternate() { f.pad("sp")   } else { f.pad("x2")  },
            Register::X3  =>
                if f.alternate() { f.pad("gp")   } else { f.pad("x3")  },
            Register::X4  =>
                if f.alternate() { f.pad("tp")   } else { f.pad("x4")  },
            Register::X5  =>
                if f.alternate() { f.pad("t0")   } else { f.pad("x5")  },
            Register::X6  =>
                if f.alternate() { f.pad("t1")   } else { f.pad("x6")  },
            Register::X7  =>
                if f.alternate() { f.pad("t2")   } else { f.pad("x7")  },
            Register::X8  =>
                if f.alternate() { f.pad("s0")   } else { f.pad("x8")  },
            Register::X9  =>
                if f.alternate() { f.pad("s1")   } else { f.pad("x9")  },
            Register::X10 =>
                if f.alternate() { f.pad("a0")   } else { f.pad("x10") },
            Register::X11 =>
                if f.alternate() { f.pad("a1")   } else { f.pad("x11") },
            Register::X12 =>
                if f.alternate() { f.pad("a2")   } else { f.pad("x12") },
            Register::X13 =>
                if f.alternate() { f.pad("a3")   } else { f.pad("x13") },
            Register::X14 =>
                if f.alternate() { f.pad("a4")   } else { f.pad("x14") },
            Register::X15 =>
                if f.alternate() { f.pad("a5")   } else { f.pad("x15") },
            Register::X16 =>
                if f.alternate() { f.pad("a6")   } else { f.pad("x16") },
            Register::X17 =>
                if f.alternate() { f.pad("a7")   } else { f.pad("x17") },
            Register::X18 =>
                if f.alternate() { f.pad("s2")   } else { f.pad("x18") },
            Register::X19 =>
                if f.alternate() { f.pad("s3")   } else { f.pad("x19") },
            Register::X20 =>
                if f.alternate() { f.pad("s4")   } else { f.pad("x20") },
            Register::X21 =>
                if f.alternate() { f.pad("s5")   } else { f.pad("x21") },
            Register::X22 =>
                if f.alternate() { f.pad("s6")   } else { f.pad("x22") },
            Register::X23 =>
                if f.alternate() { f.pad("s7")   } else { f.pad("x23") },
            Register::X24 =>
                if f.alternate() { f.pad("s8")   } else { f.pad("x24") },
            Register::X25 =>
                if f.alternate() { f.pad("s9")   } else { f.pad("x25") },
            Register::X26 =>
                if f.alternate() { f.pad("s10")  } else { f.pad("x26") },
            Register::X27 =>
                if f.alternate() { f.pad("s11")  } else { f.pad("x27") },
            Register::X28 =>
                if f.alternate() { f.pad("t3")   } else { f.pad("x28") },
            Register::X29 =>
                if f.alternate() { f.pad("t4")   } else { f.pad("x29") },
            Register::X30 =>
                if f.alternate() { f.pad("t5")   } else { f.pad("x30") },
            Register::X31 =>
                if f.alternate() { f.pad("t6")   } else { f.pad("x31") },
            Register::PC  =>
                f.pad("pc"),
        }
    }
}

impl From<Word> for Register {
    fn from(word: Word) -> Register {
        match word {
             0 => Register::X0,
             1 => Register::X1,
             2 => Register::X2,
             3 => Register::X3,
             4 => Register::X4,
             5 => Register::X5,
             6 => Register::X6,
             7 => Register::X7,
             8 => Register::X8,
             9 => Register::X9,
            10 => Register::X10,
            11 => Register::X11,
            12 => Register::X12,
            13 => Register::X13,
            14 => Register::X14,
            15 => Register::X15,
            16 => Register::X16,
            17 => Register::X17,
            18 => Register::X18,
            19 => Register::X19,
            20 => Register::X20,
            21 => Register::X21,
            22 => Register::X22,
            23 => Register::X23,
            24 => Register::X24,
            25 => Register::X25,
            26 => Register::X26,
            27 => Register::X27,
            28 => Register::X28,
            29 => Register::X29,
            30 => Register::X30,
            31 => Register::X31,
            32 => Register::PC,
            _    => Register::X0,
        }
    }
}

impl Register {
    /// Decodes a given register operand out of a full instruction word, into
    /// an internal representation.
    /// Returns None on a failure.
    pub fn extract_register(
       register: RegisterOperand,
       instruction: Word
    ) -> Option<Register> {
        let base_code = match BaseCode::from_instruction(instruction) {
            Some(c) => c,
            None    => return None,
        };
        match register {
            RegisterOperand::RD  =>
                if base_code.has_rd() {
                    Some(Register::from((instruction >> 7) & REG_OP_MASK))
                } else {
                    None
                },
            RegisterOperand::RS1 =>
                if base_code.has_rs1() {
                    Some(Register::from((instruction >> 15) & REG_OP_MASK))
                } else {
                    None
                },
            RegisterOperand::RS2 =>
                if base_code.has_rs2() {
                    Some(Register::from((instruction >> 25) & REG_OP_MASK))
                } else {
                    None
                },
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Decodes the immediate out of a full instruction word.
/// Returns None on a failure.
pub fn extract_immediate(instruction: Word) -> Option<Word> {
    let base_code = match BaseCode::from_instruction(instruction) {
        Some(c) => c,
        None    => return None,
    };
    let i = instruction;
    match Format::from(base_code) {
        Format::R => None,
        Format::I =>
            Some(sign_extend_from_msb(11, imm_ex_1(i, 20,  true) | imm_ex_2(i, 11))),
        Format::S =>
            Some(sign_extend_from_msb(11, imm_ex_1(i,  7,  true) | imm_ex_2(i, 11))),
        Format::B =>
            Some(sign_extend_from_msb(12, imm_ex_1(i,  7, false) | imm_ex_2(i, 12))),
        Format::U =>
            Some(imm_ex_3(i, 12, 31)),
        Format::J =>
            Some(sign_extend_from_msb(20, imm_ex_1(i, 20, false) | imm_ex_2(i, 20) |
                                          imm_ex_3(i, 12, 19))),
    }
}

/// Machinery to extract bits of the immediate from an instruction, and place
/// them in a word containing a partially complete immediate.
///
/// ## Extraction Modes (with offset `b`):
///
/// | **Contiguous Mode (`cont`)** | **Instruction Bits** | **Final Immediate Bits** |
/// |------------------------------|----------------------|--------------------------|
/// |              true            |      `[b+4:b+0]`     |          `[4:0]`         |
/// |             false            |    `[b+4:b+1,b+0]`   |        `[4:1,11]`        |
fn imm_ex_1(i: Word, b: u8, cont: bool) -> Word {
    let imm = (i >> b) & 0b11111;
    if !cont {
        ((imm & 0b1) << 11) | (imm & 0b11110)
    } else {
        imm
    }
}

/// Machinery to extract bits of the immediate from an instruction, and place
/// them in a word containing a partially complete immediate.
///
/// ## Extraction Schematic (with `e`dge case):
///
/// | **Instruction Bits** | **Final Immediate Bits** |
/// |----------------------|--------------------------|
/// |     `[31,30:25]`     |        `[e,10:5]`        |
///
/// _Where `e >= 11`. For all values `e <= 11`, `e` is left as `11`._
fn imm_ex_2(i: Word, e: u8) -> Word {
    let imm = (i >> 20) & 0b111111100000;
    if e > 11 {
        ((imm & 0b10000000000) << (e - 10)) | (imm & 0b011111100000)
    } else {
        imm
    }
}

/// Machinery to extract bits of the immediate from an instruction, and place
/// them in a word containing a partially complete immediate.
///
/// ## Extraction Schematic (with range `a` .. `b`):
///
/// | **Instruction Bits** | **Final Immediate Bits** |
/// |----------------------|--------------------------|
/// |        `[b:a]`       |         `[b:a]`          |
fn imm_ex_3(i: Word, a: u8, b: u8) -> Word {
    i & (((0b1 << 1+b-a) - 1) << a)
}

/// Sign extends the given `word` from the given `msb` onwards.
fn sign_extend_from_msb(msb: u8, word: Word) -> Word {
    if ((word >> msb) & 0b1) == 0b1 {
        word | (((1 << (32 - msb)) - 1) << msb)
    } else {
        word
    }
}

