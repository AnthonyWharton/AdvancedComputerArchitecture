use isa::{Instruction, UW, W};
use isa::op_code::Operation;
use isa::operand::Register;
use super::memory::Memory;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Main entry point for execution given an instruction, state and memory.
pub fn exec(inst: &Instruction, state: &mut State, memory: &mut Memory) {
    match inst.op {
        Operation::LUI    => exec_u_type(inst, state),
        Operation::AUIPC  => exec_u_type(inst, state),
        Operation::JAL    => exec_j_type(inst, state),
        Operation::JALR   => exec_j_type(inst, state),
        Operation::BEQ    => exec_b_type(inst, state), //unimplemented!
        Operation::BNE    => exec_b_type(inst, state), //unimplemented!
        Operation::BLT    => exec_b_type(inst, state), //unimplemented!
        Operation::BGE    => exec_b_type(inst, state), //unimplemented!
        Operation::BLTU   => exec_b_type(inst, state), //unimplemented!
        Operation::BGEU   => exec_b_type(inst, state), //unimplemented!
        Operation::LB     => exec_i_type(inst, state), //unimplemented!
        Operation::LH     => exec_i_type(inst, state), //unimplemented!
        Operation::LW     => exec_i_type(inst, state), //unimplemented!
        Operation::LBU    => exec_i_type(inst, state), //unimplemented!
        Operation::LHU    => exec_i_type(inst, state), //unimplemented!
        Operation::SB     => exec_s_type(inst, state), //unimplemented!
        Operation::SH     => exec_s_type(inst, state), //unimplemented!
        Operation::SW     => exec_s_type(inst, state), //unimplemented!
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
        Operation::FENCE  => exec_i_type(inst, state), //unimplemented!
        Operation::FENCEI => exec_i_type(inst, state), //unimplemented!
        Operation::ECALL  => exec_i_type(inst, state), //unimplemented!
        Operation::EBREAK => exec_i_type(inst, state), //unimplemented!
        Operation::CSRRW  => exec_i_type(inst, state), //unimplemented!
        Operation::CSRRS  => exec_i_type(inst, state), //unimplemented!
        Operation::CSRRC  => exec_i_type(inst, state), //unimplemented!
        Operation::CSRRWI => exec_i_type(inst, state), //unimplemented!
        Operation::CSRRSI => exec_i_type(inst, state), //unimplemented!
        Operation::CSRRCI => exec_i_type(inst, state), //unimplemented!
        Operation::MUL    => exec_i_type(inst, state), //unimplemented!
        Operation::MULH   => exec_i_type(inst, state), //unimplemented!
        Operation::MULHSU => exec_i_type(inst, state), //unimplemented!
        Operation::MULHU  => exec_i_type(inst, state), //unimplemented!
        Operation::DIV    => exec_i_type(inst, state), //unimplemented!
        Operation::DIVU   => exec_i_type(inst, state), //unimplemented!
        Operation::REM    => exec_i_type(inst, state), //unimplemented!
        Operation::REMU   => exec_i_type(inst, state), //unimplemented!
    }
}

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
    };

    r[Register::PC as usize] += 4;
}

/// Executes an I type instruction, modifying the borrowed state.
fn exec_i_type(inst: &Instruction, state: &mut State) {
    let rd  = inst.rd
        .expect("Invalid I type instruction (no rd) failed to execute.") as usize;
    let rs1 = inst.rs1
        .expect("Invalid I type instruction (no rs1) failed to execute.") as usize;
    let imm = inst.imm
        .expect("Invalid I type instruction (no imm) failed to execute.");
    let r = &mut state.register; // Shorthand, should hopefully be optimised out

    if inst.op == Operation::JALR {
        if rd != 0 {
            r[rd] = r[Register::PC as usize] + 4;
        }
        r[Register::PC as usize] += r[rs1] + imm;
        r[Register::PC as usize] &= !0b1;
        return;
    }

    // Early exit, assigning to 0 is a nop as there are no side effect status
    // registers at this point in time.
    if rd == 0 {
        return;
    }

    r[rd] = match inst.op {
        Operation::ADDI  => r[rs1] + imm,
        Operation::SLTI  => (r[rs1] < imm) as W,
        Operation::SLTIU => ((r[rs1] as UW) < (imm as UW)) as W,
        Operation::XORI  => r[rs1] ^ imm,
        Operation::ORI   => r[rs1] | imm,
        Operation::ANDI  => r[rs1] & imm,
        Operation::SLLI  => r[rs1] << imm,
        Operation::SRLI  => ((r[rs1] as UW) >> (imm as UW)) as W,
        Operation::SRAI  => r[rs1] >> (imm & 0b11111),
        _ => panic!("Unkown I type instruction failed to execute.")
    };

    r[Register::PC as usize] += 4;
}

/// Executes an S type instruction, modifying the borrowed state.
#[allow(unused)]
fn exec_s_type(inst: &Instruction, state: &mut State) {
    state.register[Register::PC as usize] += 4;
}

/// Executes an B type instruction, modifying the borrowed state.
#[allow(unused)]
fn exec_b_type(inst: &Instruction, state: &mut State) {
    state.register[Register::PC as usize] += 4;
}

/// Executes an U type instruction, modifying the borrowed state.
fn exec_u_type(inst: &Instruction, state: &mut State) {
    let rd  = inst.rd
        .expect("Invalid U type instruction (no rd) failed to execute.") as usize;
    let imm = inst.imm
        .expect("Invalid U type instruction (no imm) failed to execute.");
    let r = &mut state.register; // Shorthand, should hopefully be optimised out

    match inst.op {
        Operation::LUI   => if rd != 0 { r[rd] = imm },
        Operation::AUIPC => r[Register::PC as usize] += imm - 4,
        _ => panic!("Unkown U type instruction failed to execute.")
    };

    r[Register::PC as usize] += 4;
}

/// Executes an J type instruction, modifying the borrowed state.
fn exec_j_type(inst: &Instruction, state: &mut State) {
    let rd  = inst.rd
        .expect("Invalid U type instruction (no rd) failed to execute.") as usize;
    let imm = inst.imm
        .expect("Invalid U type instruction (no imm) failed to execute.");
    let r = &mut state.register; // Shorthand, should hopefully be optimised out

    if rd != 0 {
        r[rd] = r[Register::PC as usize] + 4;
    }

    match inst.op {
        Operation::JAL => r[Register::PC as usize] += imm,
        _ => panic!("Unkown U type instruction failed to execute.")
    }
}

