use isa::op_code::Operation;
use isa::operand::Register;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Main entry point for execution given an instruction, state and memory.
pub fn exec(state: &mut State) {
    match state.l_decode.op {
        Operation::LUI    => exec_u_type(state),
        Operation::AUIPC  => exec_u_type(state),
        Operation::JAL    => exec_j_type(state),
        Operation::JALR   => exec_i_type(state),
        Operation::BEQ    => exec_b_type(state),
        Operation::BNE    => exec_b_type(state),
        Operation::BLT    => exec_b_type(state),
        Operation::BGE    => exec_b_type(state),
        Operation::BLTU   => exec_b_type(state),
        Operation::BGEU   => exec_b_type(state),
        Operation::LB     => exec_i_type(state),
        Operation::LH     => exec_i_type(state),
        Operation::LW     => exec_i_type(state),
        Operation::LBU    => exec_i_type(state),
        Operation::LHU    => exec_i_type(state),
        Operation::SB     => exec_s_type(state), //unimplemented!
        Operation::SH     => exec_s_type(state), //unimplemented!
        Operation::SW     => exec_s_type(state), //unimplemented!
        Operation::ADDI   => exec_i_type(state),
        Operation::SLTI   => exec_i_type(state),
        Operation::SLTIU  => exec_i_type(state),
        Operation::XORI   => exec_i_type(state),
        Operation::ORI    => exec_i_type(state),
        Operation::ANDI   => exec_i_type(state),
        Operation::SLLI   => exec_i_type(state),
        Operation::SRLI   => exec_i_type(state),
        Operation::SRAI   => exec_i_type(state),
        Operation::ADD    => exec_r_type(state),
        Operation::SUB    => exec_r_type(state),
        Operation::SLL    => exec_r_type(state),
        Operation::SLT    => exec_r_type(state),
        Operation::SLTU   => exec_r_type(state),
        Operation::XOR    => exec_r_type(state),
        Operation::SRL    => exec_r_type(state),
        Operation::SRA    => exec_r_type(state),
        Operation::OR     => exec_r_type(state),
        Operation::AND    => exec_r_type(state),
        Operation::FENCE  => exec_i_type(state), //unimplemented!
        Operation::FENCEI => exec_i_type(state), //unimplemented!
        Operation::ECALL  => exec_i_type(state), //unimplemented!
        Operation::EBREAK => exec_i_type(state), //unimplemented!
        Operation::CSRRW  => exec_i_type(state), //unimplemented!
        Operation::CSRRS  => exec_i_type(state), //unimplemented!
        Operation::CSRRC  => exec_i_type(state), //unimplemented!
        Operation::CSRRWI => exec_i_type(state), //unimplemented!
        Operation::CSRRSI => exec_i_type(state), //unimplemented!
        Operation::CSRRCI => exec_i_type(state), //unimplemented!
        Operation::MUL    => exec_i_type(state), //unimplemented!
        Operation::MULH   => exec_i_type(state), //unimplemented!
        Operation::MULHSU => exec_i_type(state), //unimplemented!
        Operation::MULHU  => exec_i_type(state), //unimplemented!
        Operation::DIV    => exec_i_type(state), //unimplemented!
        Operation::DIVU   => exec_i_type(state), //unimplemented!
        Operation::REM    => exec_i_type(state), //unimplemented!
        Operation::REMU   => exec_i_type(state), //unimplemented!
    }
}

/// Executes an R type instruction, modifying the borrowed state.
fn exec_r_type(state: &mut State) {
    let rd  = state.l_decode.rd
        .expect("Invalid R type instruction (no rd) failed to execute.") as usize;

    // Early exit, assigning to 0 is a nop as there are no side effect status
    // registers at this point in time.
    if rd == 0 {
        return;
    }

    let rs1 = state.l_decode.rs1
        .expect("Invalid R type instruction (no rs1) failed to execute.") as usize;
    let rs2 = state.l_decode.rs2
        .expect("Invalid R type instruction (no rs2) failed to execute.") as usize;
    let r = &mut state.register; // Shorthand, should hopefully be optimised out

    r[rd] = match state.l_decode.op {
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
fn exec_i_type(state: &mut State) {
    let rd  = state.l_decode.rd
        .expect("Invalid I type instruction (no rd) failed to execute.") as usize;
    let rs1 = state.l_decode.rs1
        .expect("Invalid I type instruction (no rs1) failed to execute.") as usize;
    let imm = state.l_decode.imm
        .expect("Invalid I type instruction (no imm) failed to execute.");
    
    // Shorthand, should hopefully be optimised out
    let r = &mut state.register;
    let m = &mut state.memory;

    if state.l_decode.op == Operation::JALR {
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

    r[rd] = match state.l_decode.op {
        Operation::LB    => m[(r[rs1] + imm) as usize] as i8 as i32,
        Operation::LH    => m.read_i16((r[rs1] + imm) as usize).word as i32,
        Operation::LW    => m.read_i32((r[rs1] + imm) as usize).word,
        Operation::LBU   => m[(r[rs1] + imm) as usize] as i32,
        Operation::LHU   => m.read_u16((r[rs1] + imm) as usize).word as i32,
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
fn exec_s_type(state: &mut State) {
    let rs1 = state.l_decode.rs1
        .expect("Invalid S type instruction (no rs1) failed to execute.") as usize;
    let rs2 = state.l_decode.rs2
        .expect("Invalid S type instruction (no rs2) failed to execute.") as usize;
    let imm = state.l_decode.imm
        .expect("Invalid S type instruction (no imm) failed to execute.");
    
    // Shorthand, should hopefully be optimised out
    let r = &mut state.register;
    let m = &mut state.memory;
    let pc = Register::PC as usize;

    match state.l_decode.op {
        Operation::SB => { m[(r[rs1] + imm) as usize] = r[rs2] as u8 },
        Operation::SH => { m.write_i16((r[rs1] + imm) as usize, r[rs2] as i16); () },
        Operation::SW => { m.write_i32((r[rs1] + imm) as usize, r[rs2]); () },
        _ => panic!("Unkown s type instruction failed to execute.")
    };
    r[pc] += 4;
}

/// Executes an B type instruction, modifying the borrowed state.
fn exec_b_type(state: &mut State) {
    let rs1 = state.l_decode.rs1
        .expect("Invalid B type instruction (no rs1) failed to execute.") as usize;
    let rs2 =state.l_decode .rs2
        .expect("Invalid B type instruction (no rs2) failed to execute.") as usize;
    let imm = state.l_decode.imm
        .expect("Invalid B type instruction (no imm) failed to execute.");

    // Shorthand, should hopefully be optimised out
    let r = &mut state.register;
    let pc = Register::PC as usize;

    match state.l_decode.op {
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
fn exec_u_type(state: &mut State) {
    let rd  = state.l_decode.rd
        .expect("Invalid U type instruction (no rd) failed to execute.") as usize;
    let imm = state.l_decode.imm
        .expect("Invalid U type instruction (no imm) failed to execute.");
    
    // Shorthand, should hopefully be optimised out
    let r = &mut state.register;

    match state.l_decode.op {
        Operation::LUI   => if rd != 0 { r[rd] = imm },
        Operation::AUIPC => r[Register::PC as usize] += imm - 4,
        _ => panic!("Unkown U type instruction failed to execute.")
    };

    r[Register::PC as usize] += 4;
}

/// Executes an J type instruction, modifying the borrowed state.
fn exec_j_type(state: &mut State) {
    let rd  = state.l_decode.rd
        .expect("Invalid U type instruction (no rd) failed to execute.") as usize;
    let imm = state.l_decode.imm
        .expect("Invalid U type instruction (no imm) failed to execute.");

    // Shorthand, should hopefully be optimised out
    let r = &mut state.register;

    if rd != 0 {
        r[rd] = r[Register::PC as usize] + 4;
    }

    match state.l_decode.op {
        Operation::JAL => r[Register::PC as usize] += imm,
        _ => panic!("Unkown U type instruction failed to execute.")
    }
}

