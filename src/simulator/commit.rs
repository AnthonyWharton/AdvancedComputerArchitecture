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
        state.stats.executed += 1;
    }
    state.register.read_reg(Register::PC).unwrap() == -1
}

/// Commits an R type instruction from a reorder buffer entry to the given
/// state.
fn cm_r_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer.rob[entry];
    if rob_entry.act_pc == state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        // Write back to register file
        state
            .register
            .write_to_name(rob_entry.name_rd.unwrap(), rob_entry.act_rd);
        state
            .register
            .write_to_name(Register::PC as usize, rob_entry.act_pc);
        state
            .register
            .finished_write(rob_entry.reg_rd.unwrap(), rob_entry.name_rd.unwrap());
        if let Right(name) = rob_entry.rs1 {
            state.register.finished_read(name);
        }
        if let Right(name) = rob_entry.rs2 {
            state.register.finished_read(name);
        }
        state.debug_msg.push(format!("Committed {:?}", rob_entry));
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
    let rob_entry = &state_p.reorder_buffer.rob[entry];
    let rs1_s = match rob_entry.rs1 {
        Left(val) => val,
        Right(name) => {
            let r = state
                .register
                .read_at_name(name)
                .expect("Commit unit missing rs1!");
            state.register.finished_read(name);
            r
        }
    };
    let imm_s = rob_entry.imm.expect("Commit unit missing imm!");

    #[rustfmt::skip]
    let rd_val = match rob_entry.op {
        Operation::LB  => state.memory[(rs1_s + imm_s) as usize] as i8 as i32,
        Operation::LH  => state.memory.read_i16((rs1_s + imm_s) as usize).word as i32,
        Operation::LW  => state.memory.read_i32((rs1_s + imm_s) as usize).word,
        Operation::LBU => state.memory[(rs1_s + imm_s) as usize] as i32,
        Operation::LHU => state.memory.read_u16((rs1_s + imm_s) as usize).word as i32,
        _ => rob_entry.act_rd
    };

    // Write back to register file
    state
        .register
        .write_to_name(rob_entry.name_rd.unwrap(), rd_val);
    state
        .register
        .write_to_name(Register::PC as usize, rob_entry.act_pc);
    state
        .register
        .finished_write(rob_entry.reg_rd.unwrap(), rob_entry.name_rd.unwrap());
    // rs1 finished_read() above
    if let Right(name) = rob_entry.rs2 {
        state.register.finished_read(name);
    }

    if rob_entry.act_pc == state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        state.debug_msg.push(format!("Committed {:?}", rob_entry));
    } else {
        // Branch prediction failure
        if rob_entry.act_pc != -1 {
            state.flush_pipeline(rob_entry.act_pc as usize);
        }
        state.debug_msg.push(format!("BP FAIL, Flush {:?}", rob_entry));
    }
}

/// Commits an S type instruction from a reorder buffer entry to the given
/// state.
fn cm_s_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer.rob[entry];
    let rs1 = match rob_entry.rs1 {
        Left(val) => val,
        Right(name) => {
            let r = state
                .register
                .read_at_name(name)
                .expect("Commit unit missing rs1!");
            state.register.finished_read(name);
            r
        }
    };
    let rs2 = match rob_entry.rs2 {
        Left(val) => val,
        Right(name) => {
            let r = state
                .register
                .read_at_name(name)
                .expect("Commit unit missing rs2!");
            state.register.finished_read(name);
            r
        }
    };
    let imm = rob_entry.imm.expect("Commit unit missing imm!");

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
        state
            .register
            .write_to_name(Register::PC as usize, rob_entry.act_pc);
        state.debug_msg.push(format!("Committed {:?}", rob_entry));
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
    // Nothing to write back, just free resources
    if let Right(name) = rob_entry.rs1 {
        state.register.finished_read(name);
    }
    if let Right(name) = rob_entry.rs2 {
        state.register.finished_read(name);
    }

    if rob_entry.act_pc == state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        state
            .register
            .write_to_name(Register::PC as usize, rob_entry.act_pc);
        state.debug_msg.push(format!("Committed {:?}", rob_entry));
    } else {
        // Branch prediction failure
        if rob_entry.act_pc != -1 {
            state.flush_pipeline(rob_entry.act_pc as usize);
        }
        state.debug_msg.push(format!("BP FAIL, Flush {:?}", rob_entry));
    }
}

/// Commits an U type instruction from a reorder buffer entry to the given
/// state.
fn cm_u_type(state_p: &State, state: &mut State, entry: usize) {
    let rob_entry = &state_p.reorder_buffer.rob[entry];
    // Write back to register file
    state
        .register
        .write_to_name(rob_entry.name_rd.unwrap(), rob_entry.act_rd);
    state
        .register
        .write_to_name(Register::PC as usize, rob_entry.act_pc);
    state
        .register
        .finished_write(rob_entry.reg_rd.unwrap(), rob_entry.name_rd.unwrap());

    if rob_entry.act_pc == state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        state.debug_msg.push(format!("Committed {:?}", rob_entry));
    } else {
        // Branch prediction failure
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
    state
        .register
        .write_to_name(rob_entry.name_rd.unwrap(), rob_entry.act_rd);
    state
        .register
        .write_to_name(Register::PC as usize, rob_entry.act_pc);
    state
        .register
        .finished_write(rob_entry.reg_rd.unwrap(), rob_entry.name_rd.unwrap());

    if rob_entry.act_pc == state_p.reorder_buffer.rob[entry + 1].pc as i32 {
        state.debug_msg.push(format!("Committed {:?}", rob_entry));
    } else {
        // Branch prediction failure
        if rob_entry.act_pc != -1 {
            state.flush_pipeline(rob_entry.act_pc as usize);
        }
        state.debug_msg.push(format!("BP FAIL, Flush {:?}", rob_entry));
    }
}
