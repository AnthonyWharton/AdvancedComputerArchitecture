use either::{Left, Right};

use crate::isa::Format;
use crate::isa::op_code::Operation;
use crate::isa::operand::Register;

use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The _commit_ state in the pipeline. This is responsible for taking
/// finished instructions from the
/// ['ReorderBuffer'](../reorder/struct.ReorderBuffer.html), and then commits
/// them to the new [`State`](../state/struct.State.html).
pub fn commit_stage(state_p: &State, state: &mut State) -> bool {
    let entries = state_p
        .reorder_buffer
        .pop_finished_entries(&mut state.reorder_buffer, state_p.issue_limit);
    for entry in entries {
        let flushed = match Format::from(state_p.reorder_buffer[entry].op) {
            Format::R => cm_r_type(state_p, state, entry),
            Format::I => cm_i_type(state_p, state, entry),
            Format::S => cm_s_type(state_p, state, entry),
            Format::B => cm_b_type(state_p, state, entry),
            Format::U => cm_u_type(state_p, state, entry),
            Format::J => cm_j_type(state_p, state, entry),
        };

        // Housekeeping
        state.stats.executed += 1;

        // Early exit if finished execution or pipeline flush
        if flushed || state.register[Register::PC].data == -1 {
            break;
        }

        // Remove reference counts and bypass results from loads/stores.
        match state.reorder_buffer[entry].op {
            Operation::LB  |
            Operation::LH  |
            Operation::LW  |
            Operation::LBU |
            Operation::LHU |
            Operation::SB  |
            Operation::SH  |
            Operation::SW  => {
                // Bypass, let everyone that is waiting for this
                // register know it's value. (Lower down values).
                if let Some(rd) = state.reorder_buffer[entry].act_rd {
                    state.resv_station.execute_bypass(entry, rd);
                    state.reorder_buffer.execute_bypass(entry, rd);
                }
                // Finish with the dependencies that this was using.
                // (Higher up values).
                if let Right(name) = state.reorder_buffer[entry].rs1 {
                    state.reorder_buffer[name].ref_count -= 1;
                    state.reorder_buffer[entry].rs1 = Left(0);
                }
                if let Right(name) = state.reorder_buffer[entry].rs2 {
                    state.reorder_buffer[name].ref_count -= 1;
                    state.reorder_buffer[entry].rs2 = Left(0);
                }
            }
            _ => ()
        }
    }
    state.register[Register::PC].data == -1
}

/// Commits an R type instruction from a reorder buffer entry to the given
/// state. Returns whether a full pipeline flush occured.
fn cm_r_type(state_p: &State, state: &mut State, entry: usize) -> bool {
    let rob = &state_p.reorder_buffer;
    let rob_entry = &rob[entry];
    // Branch prediction failure check
    if rob_entry.act_pc == rob[(entry + 1) % rob.capacity].pc as i32 {
        // Write back to register file
        state.register.writeback(rob_entry.reg_rd.unwrap(), entry, rob_entry.act_rd.unwrap());
        state.register[Register::PC].data = rob_entry.act_pc;
        false
    } else {
        panic!(
            format!("Did not expect R type instruction to have mismatching PC! - {:?}", rob_entry)
        )
    }
}

/// Commits an I type instruction from a reorder buffer entry to the given
/// state. Returns whether a full pipeline flush occured.
fn cm_i_type(state_p: &State, state: &mut State, entry: usize) -> bool {
    let rob = &state_p.reorder_buffer;
    let rob_entry = &rob[entry];
    let rs1_s = match rob_entry.rs1 {
        Left(val) => val,
        Right(name) => state
            .reorder_buffer[name]
            .act_rd
            .unwrap_or(0) // Some instructions may not require this - namely
                          // those that are not loads, so fail quietly
    };
    let imm_s = rob_entry.imm.unwrap_or(0);

    #[rustfmt::skip]
    let rd_val = match rob_entry.op {
        Operation::LB  => state.memory[(rs1_s + imm_s) as usize] as i8 as i32,
        Operation::LH  => state.memory.read_i16((rs1_s + imm_s) as usize).word as i32,
        Operation::LW  => state.memory.read_i32((rs1_s + imm_s) as usize).word,
        Operation::LBU => state.memory[(rs1_s + imm_s) as usize] as i32,
        Operation::LHU => state.memory.read_u16((rs1_s + imm_s) as usize).word as i32,
        Operation::ECALL => {
            match (state.register[Register::X10].data as u8) as char {
                '\n' => {
                    state.out.push(String::new())
                }
                a if a.is_ascii_graphic() || a.is_ascii_whitespace() => {
                    let last = state.out.len() - 1;
                    state.out[last].push(a)
                }
                _ => ()
            }
            0
        }
        _ => rob_entry.act_rd.unwrap()
    };

    // Write back to register file (and ROB in case it was a load)
    state.reorder_buffer[entry].act_rd = Some(rd_val);
    state.register.writeback(rob_entry.reg_rd.unwrap(), entry, rd_val);
    state.register[Register::PC].data = rob_entry.act_pc;


    // Branch prediction update and failure check
    if rob_entry.act_pc != rob[(entry + 1) % rob.capacity].pc as i32 &&
       rob_entry.act_pc != -1 {
        if rob_entry.op == Operation::JALR {
            state.branch_predictor.commit_feedback(rob_entry, true);
        }
        state.flush_pipeline(rob_entry.act_pc as usize);
        true
    } else {
        if rob_entry.op == Operation::JALR {
            state.branch_predictor.commit_feedback(rob_entry, false);
            state.stats.bp_success += 1;
        }
        false
    }
}

/// Commits an S type instruction from a reorder buffer entry to the given
/// state. Returns whether a full pipeline flush occured.
fn cm_s_type(state_p: &State, state: &mut State, entry: usize) -> bool {
    let rob = &state_p.reorder_buffer;
    let rob_entry = &rob[entry];
    let rs1 = match rob_entry.rs1 {
        Left(val) => val,
        Right(name) => state
            .reorder_buffer[name]
            .act_rd
            .expect("Commit S-type expected rs1!"),
    };
    let rs2 = match rob_entry.rs2 {
        Left(val) => val,
        Right(name) => state
            .reorder_buffer[name]
            .act_rd
            .expect("Commit S-type expected rs2!"),
    };
    let imm = rob_entry.imm.expect("Commit S type missing imm!");

    // Write back value to memory
    match rob_entry.op {
        Operation::SB => state.memory[(rs1 + imm) as usize] = rs2 as u8,
        Operation::SH => {
            state.memory.write_i16((rs1 + imm) as usize, rs2 as i16);
            ()
        }
        Operation::SW => {
            state.memory.write_i32((rs1 + imm) as usize, rs2);
            ()
        }
        _ => panic!("Unknown S-type instruction failed to commit."),
    };

    // Branch prediction failure check
    if rob_entry.act_pc == rob[(entry + 1) % rob.capacity].pc as i32 {
        state.register[Register::PC].data = rob_entry.act_pc;
        false
    } else {
        panic!(
            format!("Did not expect S-type instruction to have mismatching PC! - {:?}", rob_entry)
        )
    }
}

/// Commits an B type instruction from a reorder buffer entry to the given
/// state. Returns whether a full pipeline flush occured.
fn cm_b_type(state_p: &State, state: &mut State, entry: usize) -> bool {
    let rob = &state_p.reorder_buffer;
    let rob_entry = &rob[entry];

    // Branch prediction update and failure check
    if rob_entry.act_pc != rob[(entry + 1) % rob.capacity].pc as i32 &&
       rob_entry.act_pc != -1 {
        state.branch_predictor.commit_feedback(rob_entry, true);
        state.flush_pipeline(rob_entry.act_pc as usize);
        true
    } else {
        state.branch_predictor.commit_feedback(rob_entry, false);
        state.register[Register::PC].data = rob_entry.act_pc;
        state.stats.bp_success += 1;
        false
    }
}

/// Commits an U type instruction from a reorder buffer entry to the given
/// state. Returns whether a full pipeline flush occured.
fn cm_u_type(state_p: &State, state: &mut State, entry: usize) -> bool {
    let rob = &state_p.reorder_buffer;
    let rob_entry = &rob[entry];
    // Write back to register file
    state.register.writeback(rob_entry.reg_rd.unwrap(), entry, rob_entry.act_rd.unwrap());
    state.register[Register::PC].data = rob_entry.act_pc;

    // Branch prediction failure
    if rob_entry.act_pc == rob[(entry + 1) % rob.capacity].pc as i32 {
        state.stats.bp_success += 1;
        false
    } else {
        panic!(
            format!("Did not expect U type instruction to have mismatching PC! - {:?}", rob_entry)
        )
    }
}

/// Commits an J type instruction from a reorder buffer entry to the given
/// state. Returns whether a full pipeline flush occured.
fn cm_j_type(state_p: &State, state: &mut State, entry: usize) -> bool {
    let rob = &state_p.reorder_buffer;
    let rob_entry = &rob[entry];

    // Write back to register file
    state.register.writeback(rob_entry.reg_rd.unwrap(), entry, rob_entry.act_rd.unwrap());
    state.register[Register::PC].data = rob_entry.act_pc;

    // Branch prediction update and failure check
    if rob_entry.act_pc != rob[(entry + 1) % rob.capacity].pc as i32 &&
       rob_entry.act_pc != -1 {
        state.branch_predictor.commit_feedback(rob_entry, true);
        state.flush_pipeline(rob_entry.act_pc as usize);
        true
    } else {
        state.branch_predictor.commit_feedback(rob_entry, false);
        state.stats.bp_success += 1;
        false
    }
}
