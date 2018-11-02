use isa::Instruction;
use isa::op_code::Operation;
use isa::operand::Register;
use super::memory::Memory;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Main entry point for execution given an instruction, state and memory.
pub fn exec(inst: &Instruction, state: &mut State, memory: &mut Memory) {
    match inst.op {
        Operation::LUI    => exec_u_type(inst, state, memory),
        Operation::AUIPC  => exec_u_type(inst, state, memory),
        Operation::JAL    => exec_j_type(inst, state, memory),
        Operation::JALR   => exec_i_type(inst, state, memory),
        Operation::BEQ    => exec_b_type(inst, state, memory),
        Operation::BNE    => exec_b_type(inst, state, memory),
        Operation::BLT    => exec_b_type(inst, state, memory),
        Operation::BGE    => exec_b_type(inst, state, memory),
        Operation::BLTU   => exec_b_type(inst, state, memory),
        Operation::BGEU   => exec_b_type(inst, state, memory),
        Operation::LB     => exec_i_type(inst, state, memory),
        Operation::LH     => exec_i_type(inst, state, memory),
        Operation::LW     => exec_i_type(inst, state, memory),
        Operation::LBU    => exec_i_type(inst, state, memory),
        Operation::LHU    => exec_i_type(inst, state, memory),
        Operation::SB     => exec_s_type(inst, state, memory), //unimplemented!
        Operation::SH     => exec_s_type(inst, state, memory), //unimplemented!
        Operation::SW     => exec_s_type(inst, state, memory), //unimplemented!
        Operation::ADDI   => exec_i_type(inst, state, memory),
        Operation::SLTI   => exec_i_type(inst, state, memory),
        Operation::SLTIU  => exec_i_type(inst, state, memory),
        Operation::XORI   => exec_i_type(inst, state, memory),
        Operation::ORI    => exec_i_type(inst, state, memory),
        Operation::ANDI   => exec_i_type(inst, state, memory),
        Operation::SLLI   => exec_i_type(inst, state, memory),
        Operation::SRLI   => exec_i_type(inst, state, memory),
        Operation::SRAI   => exec_i_type(inst, state, memory),
        Operation::ADD    => exec_r_type(inst, state, memory),
        Operation::SUB    => exec_r_type(inst, state, memory),
        Operation::SLL    => exec_r_type(inst, state, memory),
        Operation::SLT    => exec_r_type(inst, state, memory),
        Operation::SLTU   => exec_r_type(inst, state, memory),
        Operation::XOR    => exec_r_type(inst, state, memory),
        Operation::SRL    => exec_r_type(inst, state, memory),
        Operation::SRA    => exec_r_type(inst, state, memory),
        Operation::OR     => exec_r_type(inst, state, memory),
        Operation::AND    => exec_r_type(inst, state, memory),
        Operation::FENCE  => exec_i_type(inst, state, memory), //unimplemented!
        Operation::FENCEI => exec_i_type(inst, state, memory), //unimplemented!
        Operation::ECALL  => exec_i_type(inst, state, memory), //unimplemented!
        Operation::EBREAK => exec_i_type(inst, state, memory), //unimplemented!
        Operation::CSRRW  => exec_i_type(inst, state, memory), //unimplemented!
        Operation::CSRRS  => exec_i_type(inst, state, memory), //unimplemented!
        Operation::CSRRC  => exec_i_type(inst, state, memory), //unimplemented!
        Operation::CSRRWI => exec_i_type(inst, state, memory), //unimplemented!
        Operation::CSRRSI => exec_i_type(inst, state, memory), //unimplemented!
        Operation::CSRRCI => exec_i_type(inst, state, memory), //unimplemented!
        Operation::MUL    => exec_i_type(inst, state, memory), //unimplemented!
        Operation::MULH   => exec_i_type(inst, state, memory), //unimplemented!
        Operation::MULHSU => exec_i_type(inst, state, memory), //unimplemented!
        Operation::MULHU  => exec_i_type(inst, state, memory), //unimplemented!
        Operation::DIV    => exec_i_type(inst, state, memory), //unimplemented!
        Operation::DIVU   => exec_i_type(inst, state, memory), //unimplemented!
        Operation::REM    => exec_i_type(inst, state, memory), //unimplemented!
        Operation::REMU   => exec_i_type(inst, state, memory), //unimplemented!
    }
}

/// Executes an R type instruction, modifying the borrowed state.
fn exec_r_type(inst: &Instruction, state: &mut State, memory: &mut Memory) {
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
        Operation::SLT  => (r[rs1] < r[rs2]) as i32,
        Operation::SLTU => ((r[rs1] as u32) < (r[rs2] as u32)) as i32,
        Operation::XOR  => r[rs1] ^ r[rs2],
        Operation::SRL  => ((r[rs1] as u32) >> ((r[rs2] & 0b11111) as u32)) as i32,
        Operation::SRA  => r[rs1] >> (r[rs2] & 0b11111),
        Operation::OR   => r[rs1] | r[rs2],
        Operation::AND  => r[rs1] & r[rs2],
        _ => panic!("Unkown R type instruction failed to execute.")
    };

    r[Register::PC as usize] += 4;
}

/// Executes an I type instruction, modifying the borrowed state.
fn exec_i_type(inst: &Instruction, state: &mut State, memory: &mut Memory) {
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
        Operation::LB    => memory[(r[rs1] + imm) as usize] as i8 as i32,
        Operation::LH    => memory.read_i16((r[rs1] + imm) as usize).0 as i32,
        Operation::LW    => memory.read_i32((r[rs1] + imm) as usize).0,
        Operation::LBU   => memory[(r[rs1] + imm) as usize] as i32,
        Operation::LHU   => memory.read_u16((r[rs1] + imm) as usize).0 as i32,
        Operation::ADDI  => r[rs1] + imm,
        Operation::SLTI  => (r[rs1] < imm) as i32,
        Operation::SLTIU => ((r[rs1] as u32) < (imm as u32)) as i32,
        Operation::XORI  => r[rs1] ^ imm,
        Operation::ORI   => r[rs1] | imm,
        Operation::ANDI  => r[rs1] & imm,
        Operation::SLLI  => r[rs1] << imm,
        Operation::SRLI  => ((r[rs1] as u32) >> (imm as u32)) as i32,
        Operation::SRAI  => r[rs1] >> (imm & 0b11111),
        _ => panic!("Unkown I type instruction failed to execute.")
    };

    r[Register::PC as usize] += 4;
}

/// Executes an S type instruction, modifying the borrowed state.
fn exec_s_type(inst: &Instruction, state: &mut State, memory: &mut Memory) {
    let rs1 = inst.rs1
        .expect("Invalid S type instruction (no rs1) failed to execute.") as usize;
    let rs2 = inst.rs2
        .expect("Invalid S type instruction (no rs2) failed to execute.") as usize;
    let imm = inst.imm
        .expect("Invalid S type instruction (no imm) failed to execute.");
    let r = &mut state.register; // Shorthand, should hopefully be optimised out
    let pc = Register::PC as usize;

    match inst.op {
        Operation::SB => memory[(r[rs1] + imm) as usize] = r[rs2] as u8,
        Operation::SH => { memory.write_i16((r[rs1] + imm) as usize, r[rs2] as i16); () },
        Operation::SW => { memory.write_i32((r[rs1] + imm) as usize, r[rs2]); () },
        _ => panic!("Unkown s type instruction failed to execute.")
    };
    r[pc] += 4;
}

/// Executes an B type instruction, modifying the borrowed state.
fn exec_b_type(inst: &Instruction, state: &mut State, memory: &mut Memory) {
    let rs1 = inst.rs1
        .expect("Invalid B type instruction (no rs1) failed to execute.") as usize;
    let rs2 = inst.rs2
        .expect("Invalid B type instruction (no rs2) failed to execute.") as usize;
    let imm = inst.imm
        .expect("Invalid B type instruction (no imm) failed to execute.");
    let r = &mut state.register; // Shorthand, should hopefully be optimised out
    let pc = Register::PC as usize;

    match inst.op {
        Operation::BEQ => if r[rs1] == r[rs2] { r[pc] += imm; return },
        Operation::BNE => if r[rs1] != r[rs2] { r[pc] += imm; return },
        Operation::BLT => if r[rs1] <  r[rs2] { r[pc] += imm; return },
        Operation::BGE => if r[rs1] >= r[rs2] { r[pc] += imm; return },
        Operation::BLTU => if (r[rs1] as u32) <  (r[rs2] as u32) { r[pc] += imm; return },
        Operation::BGEU => if (r[rs1] as u32) >= (r[rs2] as u32) { r[pc] += imm; return },
        _ => panic!("Unkown B type instruction failed to execute.")
    };
    r[pc] += 4;
}

/// Executes an U type instruction, modifying the borrowed state.
fn exec_u_type(inst: &Instruction, state: &mut State, memory: &mut Memory) {
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
fn exec_j_type(inst: &Instruction, state: &mut State, memory: &mut Memory) {
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

