use either::{Left, Right};

use crate::isa::op_code::Operation;
use crate::isa::Format;

use super::reorder::ReorderEntry;
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// The writeback state in the pipeline. This is responsible for taking
/// finished instructions from the
/// ['ReorderBuffer'](../reorder/struct.ReorderBuffer.html), and then commit
/// them to the new state.
pub fn handle_writeback(state_p: &State, state: &mut State) {
    let entries = state_p
        .reorder_buffer
        .pop_finished_entry(&mut state.reorder_buffer);
    for entry in entries {
        match Format::from(entry.op) {
            Format::R => unimplemented!(),
            Format::I => unimplemented!(),
            Format::S => unimplemented!(),
            Format::B => unimplemented!(),
            Format::U => unimplemented!(),
            Format::J => unimplemented!(),
        }
    }
}

/// Commits an R type instruction from a reorder buffer entry to the given
/// state.
fn cm_r_type(state: &mut State, rob_entry: &ReorderEntry) {
    if rob_entry.pc == rob_entry.act_pc {
        // Write back to register file
        state
            .register
            .write_to_name(rob_entry.name_rd.unwrap(), rob_entry.act_rd);
        state
            .register
            .finished_write(rob_entry.reg_rd.unwrap(), rob_entry.name_rd.unwrap());
        if let Right(name) = rob_entry.rs1 {
            state.register.finished_read(name);
        }
        if let Right(name) = rob_entry.rs2 {
            state.register.finished_read(name);
        }
    } else {
        // Branch prediction failure
        panic!("Did not expect R type instruction to have mismatching PC!")
    }
}

/// Commits an I type instruction from a reorder buffer entry to the given
/// state.
fn cm_i_type(state: &mut State, rob_entry: &ReorderEntry) {
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

    if rob_entry.pc == rob_entry.act_pc {
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
            .finished_write(rob_entry.reg_rd.unwrap(), rob_entry.name_rd.unwrap());
        // rs1 finished_read() above
        if let Right(name) = rob_entry.rs2 {
            state.register.finished_read(name);
        }
    } else {
        // Branch prediction failure

        // TODO
    }
}

/// Commits an S type instruction from a reorder buffer entry to the given
/// state.
fn cm_s_type(state: &mut State, rob_entry: &ReorderEntry) {
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

    if rob_entry.pc == rob_entry.act_pc {
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
    } else {
        // Branch prediction failure
        panic!("Did not expect S type instruction to have mismatching PC!")
    }
}

/// Commits an B type instruction from a reorder buffer entry to the given
/// state.
fn cm_b_type(state: &mut State, rob_entry: &ReorderEntry) {
    if rob_entry.pc == rob_entry.act_pc {
        // Nothing to write back, just free resources
        if let Right(name) = rob_entry.rs1 {
            state.register.finished_read(name);
        }
        if let Right(name) = rob_entry.rs2 {
            state.register.finished_read(name);
        }
    } else {
        // Branch prediction failure

        // TODO
    }
}

/// Commits an U type instruction from a reorder buffer entry to the given
/// state.
fn cm_u_type(state: &mut State, rob_entry: &ReorderEntry) {
    if rob_entry.pc == rob_entry.act_pc {
        // Write back to register file
        state
            .register
            .write_to_name(rob_entry.name_rd.unwrap(), rob_entry.act_rd);
        state
            .register
            .finished_write(rob_entry.reg_rd.unwrap(), rob_entry.name_rd.unwrap());
    } else {
        // Branch prediction failure
        panic!("Did not expect U type instruction to have mismatching PC!")
    }
}

/// Commits an J type instruction from a reorder buffer entry to the given
/// state.
fn cm_j_type(state: &mut State, rob_entry: &ReorderEntry) {
    if rob_entry.pc == rob_entry.act_pc {
        // Write back to register file
        state
            .register
            .write_to_name(rob_entry.name_rd.unwrap(), rob_entry.act_rd);
        state
            .register
            .finished_write(rob_entry.reg_rd.unwrap(), rob_entry.name_rd.unwrap());
    } else {
        // Branch prediction failure

        // TODO
    }
}
