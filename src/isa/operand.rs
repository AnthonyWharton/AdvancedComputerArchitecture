use std::fmt;

use super::Format;
use super::op_code::{BaseCode, Decodable};

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

const REG_OP_MASK: i32 = 0b11111;

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
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            match self {
                Register::X0  => f.pad("zero"),
                Register::X1  => f.pad("ra"),
                Register::X2  => f.pad("sp"),
                Register::X3  => f.pad("gp"),
                Register::X4  => f.pad("tp"),
                Register::X5  => f.pad("t0"),
                Register::X6  => f.pad("t1"),
                Register::X7  => f.pad("t2"),
                Register::X8  => f.pad("s0"),
                Register::X9  => f.pad("s1"),
                Register::X10 => f.pad("a0"),
                Register::X11 => f.pad("a1"),
                Register::X12 => f.pad("a2"),
                Register::X13 => f.pad("a3"),
                Register::X14 => f.pad("a4"),
                Register::X15 => f.pad("a5"),
                Register::X16 => f.pad("a6"),
                Register::X17 => f.pad("a7"),
                Register::X18 => f.pad("s2"),
                Register::X19 => f.pad("s3"),
                Register::X20 => f.pad("s4"),
                Register::X21 => f.pad("s5"),
                Register::X22 => f.pad("s6"),
                Register::X23 => f.pad("s7"),
                Register::X24 => f.pad("s8"),
                Register::X25 => f.pad("s9"),
                Register::X26 => f.pad("s10"),
                Register::X27 => f.pad("s11"),
                Register::X28 => f.pad("t3"),
                Register::X29 => f.pad("t4"),
                Register::X30 => f.pad("t5"),
                Register::X31 => f.pad("t6"),
                Register::PC  => f.pad("pc"),
            }
        } else {
            let v = *self as u8;
            if v < 32 {
                f.pad((String::from("x") + &v.to_string()).as_str())
            } else {
                f.pad("pc")
            }
        }
    }
}

impl From<i32> for Register {
    #[rustfmt::skip]
    fn from(word: i32) -> Register {
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
            _  => Register::X0,
        }
    }
}

impl Register {
    /// Decodes a given register operand out of a full instruction word, into
    /// an internal representation.
    /// Returns None on a failure.
    pub fn extract_register(
       register: &RegisterOperand,
       instruction: i32
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
                    Some(Register::from((instruction >> 20) & REG_OP_MASK))
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
pub fn extract_immediate(instruction: i32) -> Option<i32> {
    let base_code = match BaseCode::from_instruction(instruction) {
        Some(c) => c,
        None    => return None,
    };
    let i = instruction;
    match Format::from(base_code) {
        Format::R => None,
        Format::I => Some(sign_extend_from_msb(11, imm_ex_1(i, 20,  true) | imm_ex_2(i, 11))),
        Format::S => Some(sign_extend_from_msb(11, imm_ex_1(i,  7,  true) | imm_ex_2(i, 11))),
        Format::B => Some(sign_extend_from_msb(12, imm_ex_1(i,  7, false) | imm_ex_2(i, 12))),
        Format::U => Some(imm_ex_3(i, 12, 31)),
        Format::J => Some(sign_extend_from_msb(20, imm_ex_1(i, 20, false) | imm_ex_2(i, 20) |
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
fn imm_ex_1(i: i32, b: u8, cont: bool) -> i32 {
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
fn imm_ex_2(i: i32, e: u8) -> i32 {
    let imm = (i >> 20) & 0b1111_1110_0000;
    if e > 11 {
        ((imm & 0b100_0000_0000) << (e - 10)) | (imm & 0b0111_1110_0000)
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
fn imm_ex_3(i: i32, a: u8, b: u8) -> i32 {
    i & (((0b1 << (1 + b - a)) - 1) << a)
}

/// Sign extends the given `word` from the given `msb` onwards.
fn sign_extend_from_msb(msb: u8, word: i32) -> i32 {
    if ((word >> msb) & 0b1) == 0b1 {
        word | (((1 << (32 - msb)) - 1) << msb)
    } else {
        word
    }
}

