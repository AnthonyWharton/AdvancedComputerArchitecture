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
        .pop_finished_entries(&mut state.reorder_buffer);
    for entry in entries {
        match Format::from(state_p.reorder_buffer[entry].op) {
            Format::R => cm_r_type(state_p, state, entry),
            Format::I => cm_i_type(state_p, state, entry),
            Format::S => cm_s_type(state_p, state, entry),
            Format::B => cm_b_type(state_p, state, entry),
            Format::U => cm_u_type(state_p, state, entry),
            Format::J => cm_j_type(state_p, state, entry),
        }

        // Ensure reference counts are deducted and that the entry is marked as
        // finished (Load/Stores will not be finished until they are committed)
        if let Right(name) = state_p.reorder_buffer[entry].rs1 {
            state.reorder_buffer[name].ref_count -= 1;
        }
        if let Right(name) = state_p.reorder_buffer[entry].rs2 {
            state.reorder_buffer[name].ref_count -= 1;
        }

        // Housekeeping
        state.stats.executed += 1;
    }
    state.register[Register::PC].data == -1
}

/// Commits an R type instruction from a reorder buffer entry to the given
/// state.
fn cm_r_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer[entry];
    if rob_entry.act_pc == state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        // Write back to register file
        state.register.writeback(rob_entry.reg_rd.unwrap(), entry, rob_entry.act_rd.unwrap());
        state.register[Register::PC].data = rob_entry.act_pc;
    } else {
        // Branch prediction failure
        panic!(
            format!("Did not expect R type instruction to have mismatching PC! - {:?}", rob_entry)
        )
    }
}

/// Commits an I type instruction from a reorder buffer entry to the given
/// state.
fn cm_i_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer[entry];
    let rs1_s = match rob_entry.rs1 {
        Left(val) => val,
        Right(name) => state
            .reorder_buffer[name]
            .act_rd
            .expect("Commit I-type expected rs1!"),
    };
    let imm_s = rob_entry.imm.expect("Commit unit missing imm!");

    #[rustfmt::skip]
    let rd_val = match rob_entry.op {
        Operation::LB  => state.memory[(rs1_s + imm_s) as usize] as i8 as i32,
        Operation::LH  => state.memory.read_i16((rs1_s + imm_s) as usize).word as i32,
        Operation::LW  => state.memory.read_i32((rs1_s + imm_s) as usize).word,
        Operation::LBU => state.memory[(rs1_s + imm_s) as usize] as i32,
        Operation::LHU => state.memory.read_u16((rs1_s + imm_s) as usize).word as i32,
        _ => rob_entry.act_rd.unwrap()
    };

    // Write back to register file (and ROB in case it was a load)
    state.reorder_buffer[entry].act_rd = Some(rd_val);
    state.register.writeback(rob_entry.reg_rd.unwrap(), entry, rd_val);
    state.register[Register::PC].data = rob_entry.act_pc;

    // Branch prediction failure
    if rob_entry.act_pc != state_p.reorder_buffer.rob[entry + 1].pc as i32 &&
       rob_entry.act_pc != -1 {
        state.flush_pipeline(rob_entry.act_pc as usize);
    }
}

/// Commits an S type instruction from a reorder buffer entry to the given
/// state.
fn cm_s_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer.rob[entry];
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
        _ => panic!("Unknown s type instruction failed to commit."),
    };

    if rob_entry.act_pc == state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        state.register[Register::PC].data = rob_entry.act_pc;
    } else {
        // Branch prediction failure
        panic!(
            format!("Did not expect S type instruction to have mismatching PC! - {:?}", rob_entry)
        )
    }
}

/// Commits an B type instruction from a reorder buffer entry to the given
/// state.
fn cm_b_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer.rob[entry];

    if rob_entry.act_pc == state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        state.register[Register::PC].data = rob_entry.act_pc;
    } else {
        // Branch prediction failure
        if rob_entry.act_pc != -1 {
            state.flush_pipeline(rob_entry.act_pc as usize);
        }
    }
}

/// Commits an U type instruction from a reorder buffer entry to the given
/// state.
fn cm_u_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer.rob[entry];
    // Write back to register file
    state.register.writeback(rob_entry.reg_rd.unwrap(), entry, rob_entry.act_rd.unwrap());
    state.register[Register::PC].data = rob_entry.act_pc;

    // Branch prediction failure
    if rob_entry.act_pc != state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        panic!(
            format!("Did not expect U type instruction to have mismatching PC! - {:?}", rob_entry)
        )
    }
}

/// Commits an J type instruction from a reorder buffer entry to the given
/// state.
fn cm_j_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer.rob[entry];
    // Write back to register file
    state.register.writeback(rob_entry.reg_rd.unwrap(), entry, rob_entry.act_rd.unwrap());
    state.register[Register::PC].data = rob_entry.act_pc;

    // Branch prediction failure
    if rob_entry.act_pc != state_p.reorder_buffer.rob[entry + 1].pc as i32 &&
       rob_entry.act_pc != -1 {
        state.flush_pipeline(rob_entry.act_pc as usize);
    }
}
