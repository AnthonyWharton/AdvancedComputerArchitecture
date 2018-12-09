use crate::isa::op_code::Operation;
use crate::isa::operand::Register;
use crate::isa::{Format, Instruction};

use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The writeback state in the pipeline. This is responsible for taking
/// finished instructions from the
/// ['ReorderBuffer'](../reorder/struct.ReorderBuffer.html), and then commit
/// them to the new state.
pub fn handle_writeback(_state_p: &State, _state_n: &mut State) {
    match Format::from(Operation::ADD) {
        Format::R => unimplemented!(),
        Format::I => unimplemented!(),
        Format::S => unimplemented!(),
        Format::B => unimplemented!(),
        Format::U => unimplemented!(),
        Format::J => unimplemented!(),
    }
}

// /// Writes back an R type instruction.
// fn wb_r_type(state: &mut State) {
//     let rs1s = match r.rs1 {
//         Left(val)   => val,
//         Right(name) => rf.read_at_name(name)
//             .expect("Execute unit missing rs1!"),
//     };
//     let rs2s = match r.rs2 {
//         Left(val)   => val,
//         Right(name) => rf.read_at_name(name)
//             .expect("Execute unit missing rs2!"),
//     };
//     let rs1u = rs1s as u32;
//     let rs2u = rs2s as u32;
//     let rd_val = match r.op {
//         Operation::ADD    => rs1s.overflowing_add(rs2s).0,
//         Operation::SUB    => rs1s.overflowing_sub(rs2s).0,
//         Operation::SLL    => rs1s << (rs2s & 0b11111),
//         Operation::SLT    => (rs1s < rs2s) as i32,
//         Operation::SLTU   => (rs1u < rs2u) as i32,
//         Operation::XOR    => rs1s ^ rs2s,
//         Operation::SRL    => (rs1u >> (rs2u & 0b11111)) as i32,
//         Operation::SRA    => rs1s >> (rs2s & 0b11111),
//         Operation::OR     => rs1s | rs2s,
//         Operation::AND    => rs1s & rs2s,
//         Operation::MUL    => rs1s.overflowing_mul(rs2s).0,
//         Operation::MULH   => ((i64::from(rs1s) * i64::from(rs2s)) >> 32) as i32,
//         Operation::MULHU  => ((u64::from(rs1u) * u64::from(rs2u)) >> 32) as i32,
//         Operation::MULHSU => ((i64::from(rs1s) * i64::from(rs2u)) >> 32) as i32,
//         Operation::DIV    => match rs2s {
//                                  0  => -1i32,
//                                  _  => match rs1s.overflowing_div(rs2s) {
//                                      (_, true) => i32::min_value(),
//                                      (v, _)    => v,
//                                  },
//                              },
//         Operation::DIVU   => match rs2s {
//                                  0  => i32::max_value(),
//                                  _  => (rs1u / rs2u) as i32,
//                              },
//         Operation::REM    => match rs2s {
//                                  0 => rs1s,
//                                  _ => match rs1s.overflowing_div(rs2s) {
//                                      (_, true) => 0,
//                                      (v, _)    => v,
//                                  }
//                              },
//         Operation::REMU   => match rs2s {
//                                  0 => rs1s,
//                                  _ => (rs1u % rs2u) as i32,
//                              },
//         _ => panic!("Unknown R type instruction failed to execute.")
//     };

//     self.executing.push_back((
//         ExecuteLatch {
//             rob_entry: r.rob_entry,
//             pc: rf.read_reg(Register::PC).unwrap() + 4,
//             rd: Some(rd_val),
//         },
//         ExecutionLen::from(r.op)
//     ))
// }

// /// Writes back an I type instruction.
// fn wb_i_type(&mut self, rf: &RegisterFile, r: &Reservation) {
//     let rs1 = match r.rs1 {
//         Left(val)   => val,
//         Right(name) => rf.read_at_name(name)
//             .expect("Execute unit missing rs1!"),
//     };
//     let imm = r.imm.expect("Execute unit missing imm!");

//     let rd_val = match r.op {
//         Operation::JALR   => Some(rf.read_reg(Register::PC).unwrap() + 4),
//         // TODO Move to writeback stage
//         // Operation::LB     => m[(rs1 + imm) as usize] as i8 as i32,
//         // Operation::LH     => m.read_i16((rs1 + imm) as usize).word as i32,
//         // Operation::LW     => m.read_i32((rs1 + imm) as usize).word,
//         // Operation::LBU    => m[(rs1 + imm) as usize] as i32,
//         // Operation::LHU    => m.read_u16((rs1 + imm) as usize).word as i32,
//         Operation::LB     => None,
//         Operation::LH     => None,
//         Operation::LW     => None,
//         Operation::LBU    => None,
//         Operation::LHU    => None,
//         Operation::ADDI   => Some(rs1 + imm),
//         Operation::SLTI   => Some((rs1 < imm) as i32),
//         Operation::SLTIU  => Some(((rs1 as u32) < (imm as u32)) as i32),
//         Operation::XORI   => Some(rs1 ^ imm),
//         Operation::ORI    => Some(rs1 | imm),
//         Operation::ANDI   => Some(rs1 & imm),
//         Operation::SLLI   => Some(rs1 << imm),
//         Operation::SRLI   => Some(((rs1 as u32) >> (imm as u32)) as i32),
//         Operation::SRAI   => Some(rs1 >> (imm & 0b11111)),
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

//     let pc_val = if r.op == Operation::JALR {
//         (rs1 + imm) & !0b1
//     } else {
//         rf.read_reg(Register::PC).unwrap() + 4
//     };

//     self.executing.push_back((
//         ExecuteLatch {
//             rob_entry: r.rob_entry,
//             pc: pc_val,
//             rd: rd_val,
//         },
//         ExecutionLen::from(r.op)
//     ))
// }

// /// Writes back an S type instruction.
// fn wb_s_type(&mut self, rf: &RegisterFile, r: &Reservation) {
//     // let rs1 = match r.rs1 {
//     //     Left(val)   => val,
//     //     Right(name) => rf.read_at_name(name)
//     //         .expect("Execute unit missing rs1!"),
//     // };
//     // let rs2 = match r.rs2 {
//     //     Left(val)   => val,
//     //     Right(name) => rf.read_at_name(name)
//     //         .expect("Execute unit missing rs2!"),
//     // };
//     // let imm = r.imm.expect("Execute unit missing imm!");

//     match r.op {
//         // TODO Move to writeback stage
//         // Operation::SB => { m[(rs1 + imm) as usize] = rs2 as u8 },
//         // Operation::SH => { m.write_i16((rs1 + imm) as usize, rs2 as i16); () },
//         // Operation::SW => { m.write_i32((rs1 + imm) as usize, rs2); () },
//         Operation::SB => (),
//         Operation::SH => (),
//         Operation::SW => (),
//         _ => panic!("Unknown s type instruction failed to execute.")
//     };

//     self.executing.push_back((
//         ExecuteLatch {
//             rob_entry: r.rob_entry,
//             pc: rf.read_reg(Register::PC).unwrap() + 4,
//             rd: None,
//         },
//         ExecutionLen::from(r.op)
//     ))
// }

// /// Writes back an B type instruction.
// fn wb_b_type(&mut self, rf: &RegisterFile, r: &Reservation) {
//     let rs1 = match r.rs1 {
//         Left(val)   => val,
//         Right(name) => rf.read_at_name(name)
//             .expect("Execute unit missing rs1!"),
//     };
//     let rs2 = match r.rs2 {
//         Left(val)   => val,
//         Right(name) => rf.read_at_name(name)
//             .expect("Execute unit missing rs2!"),
//     };
//     let imm = r.imm.expect("Execute unit missing imm!");

//     let pc_val = rf.read_reg(Register::PC).unwrap() + match r.op {
//         Operation::BEQ  => if rs1 == rs2 { imm } else { 4 },
//         Operation::BNE  => if rs1 != rs2 { imm } else { 4 },
//         Operation::BLT  => if rs1 <  rs2 { imm } else { 4 },
//         Operation::BGE  => if rs1 >= rs2 { imm } else { 4 },
//         Operation::BLTU =>
//             if (rs1 as u32) <  (rs2 as u32) { imm } else { 4 },
//         Operation::BGEU =>
//             if (rs1 as u32) >= (rs2 as u32) { imm } else { 4 },
//         _ => panic!("Unknown B type instruction failed to execute.")
//     };

//     self.executing.push_back((
//         ExecuteLatch {
//             rob_entry: r.rob_entry,
//             pc: pc_val,
//             rd: None,
//         },
//         ExecutionLen::from(r.op)
//     ))
// }

// /// Writes an U type instruction.
// fn wb_u_type(&mut self, rf: &RegisterFile, r: &Reservation) {
//     let imm = r.imm.expect("Execute unit missing imm!");

//     let rd_val = match r.op {
//         Operation::LUI   => Some(imm),
//         Operation::AUIPC => None,
//         _ => panic!("Unknown U type instruction failed to execute.")
//     };

//     let pc_val = rf.read_reg(Register::PC).unwrap() + match r.op {
//         Operation::LUI   => 4,
//         Operation::AUIPC => imm,
//         _ => panic!("Unknown U type instruction failed to execute.")
//     };

//     self.executing.push_back((
//         ExecuteLatch {
//             rob_entry: r.rob_entry,
//             pc: pc_val,
//             rd: rd_val,
//         },
//         ExecutionLen::from(r.op)
//     ))
// }

// /// Write back an J type instruction.
// fn wb_j_type(&mut self, rf: &RegisterFile, r: &Reservation) {
//     let imm = r.imm.expect("Execute unit missing imm!");

//     match r.op {
//         Operation::JALR => {
//             let old_pc = rf.read_reg(Register::PC).unwrap();
//             self.executing.push_back((
//                 ExecuteLatch {
//                     rob_entry: r.rob_entry,
//                     pc: old_pc + imm,
//                     rd: Some(old_pc + 4),
//                 },
//                 ExecutionLen::from(r.op)
//             ))
//         },
//         _ => panic!("Unknown J type instruction failed to execute."),
//     }
// }
