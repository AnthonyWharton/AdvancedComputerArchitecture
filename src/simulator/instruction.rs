use std::ops;

use isa::{Instruction, UW, W};
use isa::op_code::Operation;
use isa::operand::Register;
use super::memory::Memory;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Executes an R type instruction, modifying the borrowed state.
fn exec_r_type(inst: &Instruction, state: &mut State) {
    let rd  = inst.rd
        .expect("Invalid R type instruction (no rd) failed to execute.") as usize;

    // Early exit, assigning to 0 is a nop as there are no side effect status
    // registers at this point in time.
    if rd == 0 {
        return;
    }

    let rs1 = inst.rs1
        .expect("Invalid R type instruction (no rs1) failed to execute.") as usize;
    let rs2 = inst.rs2
        .expect("Invalid R type instruction (no rs2) failed to execute.") as usize;
    let imm = inst.imm
        .expect("Invalid I type instruction (no imm) failed to execute.");
    let r = &mut state.register; // Shorthand, should hopefully be optimised out

    r[rd] = match inst.op {
        Operation::ADD  => r[rs1].overflowing_add(r[rs2]).0,
        Operation::SUB  => r[rs1].overflowing_sub(r[rs2]).0,
        Operation::SLL  => r[rs1] << (r[rs2] & 0b11111),
        Operation::SLT  => (r[rs1] < r[rs2]) as W,
        Operation::SLTU => ((r[rs1] as UW) < (r[rs2] as UW)) as W,
        Operation::XOR  => r[rs1] ^ r[rs2],
        Operation::SRL  => ((r[rs1] as UW) >> ((r[rs2] & 0b11111) as UW)) as W,
        Operation::SRA  => r[rs1] >> (r[rs2] & 0b11111),
        Operation::OR   => r[rs1] | r[rs2],
        Operation::AND  => r[rs1] & r[rs2],
        _ => panic!("Unkown I type instruction failed to execute.")
    }
}

/// Executes an I type instruction, modifying the borrowed state.
fn exec_i_type(inst: &Instruction, state: &mut State) {
    let rd  = inst.rd
        .expect("Invalid I type instruction (no rd) failed to execute.") as usize;

    // Early exit, assigning to 0 is a nop as there are no side effect status
    // registers at this point in time.
    if rd == 0 {
        return;
    }

    let rs1 = inst.rs1
        .expect("Invalid I type instruction (no rs1) failed to execute.") as usize;
    let imm = inst.imm
        .expect("Invalid I type instruction (no imm) failed to execute.");

    state.register[rd] = match inst.op {
        Operation::ADDI  => state.register[rs1] + imm,
        Operation::SLTI  => (state.register[rs1] < imm) as W,
        Operation::SLTIU => ((state.register[rs1] as UW) < (imm as UW)) as W,
        Operation::XORI  => state.register[rs1] ^ imm,
        Operation::ORI   => state.register[rs1] | imm,
        Operation::ANDI  => state.register[rs1] & imm,
        Operation::SLLI  => state.register[rs1] << imm,
        Operation::SRLI  => ((state.register[rs1] as UW) >> (imm as UW)) as W,
        Operation::SRAI  => state.register[rs1] >> (imm & 0b11111),
        _ => panic!("Unkown I type instruction failed to execute.")
    }
}

/// Executes an S type instruction, modifying the borrowed state.
fn exec_s_type(inst: &Instruction, state: &mut State) {
}

/// Executes an B type instruction, modifying the borrowed state.
fn exec_b_type(inst: &Instruction, state: &mut State) {
}

/// Executes an U type instruction, modifying the borrowed state.
fn exec_u_type(inst: &Instruction, state: &mut State) {
    let rd  = inst.rd
        .expect("Invalid U type instruction (no rd) failed to execute.") as usize;
    let imm = inst.imm
        .expect("Invalid U type instruction (no imm) failed to execute.");

    match inst.op {
        Operation::LUI   => if (rd != 0) { state.register[rd] = imm },
        Operation::AUIPC => state.register[Register::PC as usize] += imm,
        _ => panic!("Unkown U type instruction failed to execute.")
    }
}

/// Executes an J type instruction, modifying the borrowed state.
fn exec_j_type(inst: &Instruction, state: &mut State) {

}

pub fn exec(inst: &Instruction, state: &mut State, memory: &mut Memory) {
    match inst.op {
        Operation::LUI    => unimplemented!(),
        Operation::AUIPC  => unimplemented!(),
        Operation::JAL    => unimplemented!(),
        Operation::JALR   => unimplemented!(),
        Operation::BEQ    => unimplemented!(),
        Operation::BNE    => unimplemented!(),
        Operation::BLT    => unimplemented!(),
        Operation::BGE    => unimplemented!(),
        Operation::BLTU   => unimplemented!(),
        Operation::BGEU   => unimplemented!(),
        Operation::LB     => unimplemented!(),
        Operation::LH     => unimplemented!(),
        Operation::LW     => unimplemented!(),
        Operation::LBU    => unimplemented!(),
        Operation::LHU    => unimplemented!(),
        Operation::SB     => unimplemented!(),
        Operation::SH     => unimplemented!(),
        Operation::SW     => unimplemented!(),
        Operation::ADDI   => exec_i_type(inst, state),
        Operation::SLTI   => exec_i_type(inst, state),
        Operation::SLTIU  => exec_i_type(inst, state),
        Operation::XORI   => exec_i_type(inst, state),
        Operation::ORI    => exec_i_type(inst, state),
        Operation::ANDI   => exec_i_type(inst, state),
        Operation::SLLI   => exec_i_type(inst, state),
        Operation::SRLI   => exec_i_type(inst, state),
        Operation::SRAI   => exec_i_type(inst, state),
        Operation::ADD    => exec_r_type(inst, state),
        Operation::SUB    => exec_r_type(inst, state),
        Operation::SLL    => exec_r_type(inst, state),
        Operation::SLT    => exec_r_type(inst, state),
        Operation::SLTU   => exec_r_type(inst, state),
        Operation::XOR    => exec_r_type(inst, state),
        Operation::SRL    => exec_r_type(inst, state),
        Operation::SRA    => exec_r_type(inst, state),
        Operation::OR     => exec_r_type(inst, state),
        Operation::AND    => exec_r_type(inst, state),
        Operation::FENCE  => unimplemented!(),
        Operation::FENCEI => unimplemented!(),
        Operation::ECALL  => unimplemented!(),
        Operation::EBREAK => unimplemented!(),
        Operation::CSRRW  => unimplemented!(),
        Operation::CSRRS  => unimplemented!(),
        Operation::CSRRC  => unimplemented!(),
        Operation::CSRRWI => unimplemented!(),
        Operation::CSRRSI => unimplemented!(),
        Operation::CSRRCI => unimplemented!(),
        Operation::MUL    => unimplemented!(),
        Operation::MULH   => unimplemented!(),
        Operation::MULHSU => unimplemented!(),
        Operation::MULHU  => unimplemented!(),
        Operation::DIV    => unimplemented!(),
        Operation::DIVU   => unimplemented!(),
        Operation::REM    => unimplemented!(),
        Operation::REMU   => unimplemented!(),
    }

    state.register[Register::PC as usize] += 4;
}
