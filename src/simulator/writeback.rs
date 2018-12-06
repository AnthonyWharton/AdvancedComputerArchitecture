use isa::{Format, Instruction};
use isa::op_code::Operation;
use isa::operand::Register;
use simulator::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The writeback state in the pipeline. This is responsible for taking
/// finished instructions from the
/// ['ReorderBuffer'](../reorder/struct.ReorderBuffer.html), and then commit
/// them to the new state.
pub fn handle_writeback(
    state_p: &State,
    state_n: &mut State,
) {
    match Format::from(Operation::ADD) {
        Format::R => unimplemented!(),
        Format::I => unimplemented!(),
        Format::S => unimplemented!(),
        Format::B => unimplemented!(),
        Format::U => unimplemented!(),
        Format::J => unimplemented!(),
    }
}

// /// Writeback for an R type instruction, modifying the borrowed state.
// fn wb_r_type(state: &mut State, instruction: Instruction) {
//     let rd  = instruction.rd
//         .expect("Invalid R type instruction (no rd) failed to execute.") as usize;

//     // Early exit, assigning to 0 is a nop as there are no side effect status
//     // registers at this point in time.
//     if rd == 0 {
//         return;
//     }

//     let rs1 = instruction.rs1
//         .expect("Invalid R type instruction (no rs1) failed to execute.") as usize;
//     let rs2 = instruction.rs2
//         .expect("Invalid R type instruction (no rs2) failed to execute.") as usize;
//     let r = &mut state.register; // Shorthand, should hopefully be optimised out

//     r[rd] = match instruction.op {
//         Operation::ADD    => r[rs1].overflowing_add(r[rs2]).0,
//         Operation::SUB    => r[rs1].overflowing_sub(r[rs2]).0,
//         Operation::SLL    => r[rs1] << (r[rs2] & 0b11111),
//         Operation::SLT    => (r[rs1] < r[rs2]) as i32,
//         Operation::SLTU   => ((r[rs1] as u32) < (r[rs2] as u32)) as i32,
//         Operation::XOR    => r[rs1] ^ r[rs2],
//         Operation::SRL    => ((r[rs1] as u32) >> ((r[rs2] & 0b11111) as u32)) as i32,
//         Operation::SRA    => r[rs1] >> (r[rs2] & 0b11111),
//         Operation::OR     => r[rs1] | r[rs2],
//         Operation::AND    => r[rs1] & r[rs2],
//         Operation::MUL    => r[rs1].overflowing_mul(r[rs2]).0,
//         Operation::MULH   => (((r[rs1] as i64) * (r[rs2] as i64)) >> 32) as i32,
//         Operation::MULHU  => (((r[rs1] as u64) * (r[rs2] as u64)) >> 32) as i32,
//         Operation::MULHSU => (((r[rs1] as i64) * (r[rs2] as i64).abs()) >> 32) as i32,
//         Operation::DIV    => match r[rs2] {
//                                  0  => -1i32,
//                                  _  => match r[rs1].overflowing_div(r[rs2]) {
//                                      (_, true) => i32::min_value(),
//                                      (v, _)    => v,
//                                  },
//                              },
//         Operation::DIVU   => match r[rs2] {
//                                  0  => i32::max_value(),
//                                  _  => ((r[rs1] as u32) / (r[rs2] as u32)) as i32,
//                              },
//         Operation::REM    => match r[rs2] {
//                                  0 => r[rs1],
//                                  _ => match r[rs1].overflowing_div(r[rs2]) {
//                                      (_, true) => 0,
//                                      (v, _)    => v,
//                                  }
//                              },
//         Operation::REMU   => match r[rs2] {
//                                  0 => r[rs1],
//                                  _ => ((r[rs1] as u32) % (r[rs2] as u32)) as i32,
//                              },
//         _ => panic!("Unknown R type instruction failed to execute.")
//     };

//     r[Register::PC as usize] += 4;
// }

// /// Writeback for an I type instruction, modifying the borrowed state.
// fn wb_i_type(state: &mut State, instruction: Instruction) {
//     let rd  = instruction.rd
//         .expect("Invalid I type instruction (no rd) failed to execute.") as usize;
//     let rs1 = instruction.rs1
//         .expect("Invalid I type instruction (no rs1) failed to execute.") as usize;
//     let imm = instruction.imm
//         .expect("Invalid I type instruction (no imm) failed to execute.");

//     // Shorthand, should hopefully be optimised out
//     let r = &mut state.register;
//     let m = &mut state.memory;

//     if instruction.op == Operation::JALR {
//         if rd != 0 {
//             r[rd] = r[Register::PC as usize] + 4;
//         }
//         r[Register::PC as usize] += r[rs1] + imm;
//         r[Register::PC as usize] &= !0b1;
//         return;
//     }

//     // Early exit, assigning to 0 is a nop as there are no side effect status
//     // registers at this point in time.
//     if rd == 0 {
//         return;
//     }

//     r[rd] = match instruction.op {
//         Operation::LB     => m[(r[rs1] + imm) as usize] as i8 as i32,
//         Operation::LH     => m.read_i16((r[rs1] + imm) as usize).word as i32,
//         Operation::LW     => m.read_i32((r[rs1] + imm) as usize).word,
//         Operation::LBU    => m[(r[rs1] + imm) as usize] as i32,
//         Operation::LHU    => m.read_u16((r[rs1] + imm) as usize).word as i32,
//         Operation::ADDI   => r[rs1] + imm,
//         Operation::SLTI   => (r[rs1] < imm) as i32,
//         Operation::SLTIU  => ((r[rs1] as u32) < (imm as u32)) as i32,
//         Operation::XORI   => r[rs1] ^ imm,
//         Operation::ORI    => r[rs1] | imm,
//         Operation::ANDI   => r[rs1] & imm,
//         Operation::SLLI   => r[rs1] << imm,
//         Operation::SRLI   => ((r[rs1] as u32) >> (imm as u32)) as i32,
//         Operation::SRAI   => r[rs1] >> (imm & 0b11111),
//         Operation::FENCE  => unimplemented!(),
//         Operation::FENCEI => unimplemented!(),
//         Operation::ECALL  => unimplemented!(),
//         Operation::EBREAK => unimplemented!(),
//         Operation::CSRRW  => unimplemented!(),
//         Operation::CSRRS  => unimplemented!(),
//         Operation::CSRRC  => unimplemented!(),
//         Operation::CSRRWI => unimplemented!(),
//         Operation::CSRRSI => unimplemented!(),
//         Operation::CSRRCI => unimplemented!(),
//         _ => panic!("Unknown I type instruction failed to execute.")
//     };

//     r[Register::PC as usize] += 4;
// }

// /// Writeback for an S type instruction, modifying the borrowed state.
// fn wb_s_type(state: &mut State, instruction: Instruction) {
//     let rs1 = instruction.rs1
//         .expect("Invalid S type instruction (no rs1) failed to execute.") as usize;
//     let rs2 = instruction.rs2
//         .expect("Invalid S type instruction (no rs2) failed to execute.") as usize;
//     let imm = instruction.imm
//         .expect("Invalid S type instruction (no imm) failed to execute.");

//     // Shorthand, should hopefully be optimised out
//     let r = &mut state.register;
//     let m = &mut state.memory;
//     let pc = Register::PC as usize;

//     match instruction.op {
//         Operation::SB => { m[(r[rs1] + imm) as usize] = r[rs2] as u8 },
//         Operation::SH => { m.write_i16((r[rs1] + imm) as usize, r[rs2] as i16); () },
//         Operation::SW => { m.write_i32((r[rs1] + imm) as usize, r[rs2]); () },
//         _ => panic!("Unknown s type instruction failed to execute.")
//     };
//     r[pc] += 4;
// }

// /// Writeback for an B type instruction, modifying the borrowed state.
// fn wb_b_type(state: &mut State, instruction: Instruction) {
//     let rs1 = instruction.rs1
//         .expect("Invalid B type instruction (no rs1) failed to execute.") as usize;
//     let rs2 =instruction .rs2
//         .expect("Invalid B type instruction (no rs2) failed to execute.") as usize;
//     let imm = instruction.imm
//         .expect("Invalid B type instruction (no imm) failed to execute.");

//     // Shorthand, should hopefully be optimised out
//     let r = &mut state.register;
//     let pc = Register::PC as usize;

//     match instruction.op {
//         Operation::BEQ => if r[rs1] == r[rs2] { r[pc] += imm; return },
//         Operation::BNE => if r[rs1] != r[rs2] { r[pc] += imm; return },
//         Operation::BLT => if r[rs1] <  r[rs2] { r[pc] += imm; return },
//         Operation::BGE => if r[rs1] >= r[rs2] { r[pc] += imm; return },
//         Operation::BLTU => if (r[rs1] as u32) <  (r[rs2] as u32) { r[pc] += imm; return },
//         Operation::BGEU => if (r[rs1] as u32) >= (r[rs2] as u32) { r[pc] += imm; return },
//         _ => panic!("Unknown B type instruction failed to execute.")
//     };
//     r[pc] += 4;
// }

// /// Writeback for an U type instruction, modifying the borrowed state.
// fn wb_u_type(state: &mut State, instruction: Instruction) {
//     let rd  = instruction.rd
//         .expect("Invalid U type instruction (no rd) failed to execute.") as usize;
//     let imm = instruction.imm
//         .expect("Invalid U type instruction (no imm) failed to execute.");

//     // Shorthand, should hopefully be optimised out
//     let r = &mut state.register;

//     match instruction.op {
//         Operation::LUI   => if rd != 0 { r[rd] = imm },
//         Operation::AUIPC => r[Register::PC as usize] += imm - 4,
//         _ => panic!("Unknown U type instruction failed to execute.")
//     };

//     r[Register::PC as usize] += 4;
// }

// /// Writeback for an J type instruction, modifying the borrowed state.
// fn wb_j_type(state: &mut State, instruction: Instruction) {
//     let rd  = instruction.rd
//         .expect("Invalid U type instruction (no rd) failed to execute.") as usize;
//     let imm = instruction.imm
//         .expect("Invalid U type instruction (no imm) failed to execute.");

//     // Shorthand, should hopefully be optimised out
//     let r = &mut state.register;

//     if rd != 0 {
//         r[rd] = r[Register::PC as usize] + 4;
//     }

//     match instruction.op {
//         Operation::JAL => r[Register::PC as usize] += imm,
//         _ => panic!("Unknown U type instruction failed to execute.")
//     }
// }
