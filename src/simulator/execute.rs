use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};

use either::{Left, Right};

use crate::isa::op_code::Operation;
use crate::isa::Format;

use super::reorder::ReorderBuffer;
use super::reservation::{ResvStation, Reservation};
use super::state::State;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// An enumeration of the different types of execute units that exist within
/// the simulator.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnitType {
    /// **Arithmentic Logic Unit**, Responsible for all arithmetic and logic
    /// operations.
    ALU,
    /// **Branch Logic Unit**, Responsible for any operations that will touch
    /// the program counter, causing the program to jump or branch to other
    /// instructions.
    BLU,
    /// **Memory & Control Unit**, Responsible for load and store operations
    /// that happen with main memory in order, as well as control operations
    /// and system calls which also need to occur in order at the writeback
    /// stage.
    MCU,
}

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// An Execute Unit is responsible for executing an instruction inside the
/// simulated processor - it's the brains of the operation! In this
/// implmentmentation, all Execute Unit's are made out of one of these objects,
/// but instantiated differently depending on the type of execute unit they
/// are, and will then accordingly behave differently.
#[derive(Clone, Debug)]
pub struct ExecuteUnit {
    /// The type of execute unit this instantiated struct is.
    pub unit_type: UnitType,
    /// The depth of the pipeline size for this execute unit. A value of 1 is
    /// a non-pipelined unit.
    pub pipeline_size: usize,
    /// The pipeline of executing instructions, and how many cycles left in the
    /// execution of the instruction.
    pub executing: VecDeque<(ExecuteResult, ExecutionLen)>,
}

/// The resulting bus that holds the results from the execute unit upon
/// completion. The execute unit will write directly to the reorder buffer.
#[derive(Copy, Clone, Debug)]
pub struct ExecuteResult {
    /// The reorder buffer entry that the result is associated with.
    pub rob_entry: usize,
    /// The new program counter after the execution.
    pub pc: i32,
    /// The new value of the `rd` result register for the execution (if
    /// applicable).
    pub rd: Option<i32>,
}

/// A collection of information regarding how long an execution will take, and
/// whether or not it blocks the pipeline.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecutionLen {
    pub blocking: bool,
    pub steps: u8,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl From<Operation> for ExecutionLen {
    #[rustfmt::skip]
    fn from(op: Operation) -> ExecutionLen {
        match op {
            Operation::LUI    => ExecutionLen { blocking: false, steps: 1 },
            Operation::AUIPC  => ExecutionLen { blocking: false, steps: 1 },
            Operation::JAL    => ExecutionLen { blocking: false, steps: 1 },
            Operation::JALR   => ExecutionLen { blocking: false, steps: 1 },
            Operation::BEQ    => ExecutionLen { blocking: false, steps: 1 },
            Operation::BNE    => ExecutionLen { blocking: false, steps: 1 },
            Operation::BLT    => ExecutionLen { blocking: false, steps: 1 },
            Operation::BGE    => ExecutionLen { blocking: false, steps: 1 },
            Operation::BLTU   => ExecutionLen { blocking: false, steps: 1 },
            Operation::BGEU   => ExecutionLen { blocking: false, steps: 1 },
            Operation::LB     => ExecutionLen { blocking: true, steps: 3 },
            Operation::LH     => ExecutionLen { blocking: true, steps: 3 },
            Operation::LW     => ExecutionLen { blocking: true, steps: 3 },
            Operation::LBU    => ExecutionLen { blocking: true, steps: 3 },
            Operation::LHU    => ExecutionLen { blocking: true, steps: 3 },
            Operation::SB     => ExecutionLen { blocking: true, steps: 3 },
            Operation::SH     => ExecutionLen { blocking: true, steps: 3 },
            Operation::SW     => ExecutionLen { blocking: true, steps: 3 },
            Operation::ADDI   => ExecutionLen { blocking: false, steps: 1 },
            Operation::SLTI   => ExecutionLen { blocking: false, steps: 1 },
            Operation::SLTIU  => ExecutionLen { blocking: false, steps: 1 },
            Operation::XORI   => ExecutionLen { blocking: false, steps: 1 },
            Operation::ORI    => ExecutionLen { blocking: false, steps: 1 },
            Operation::ANDI   => ExecutionLen { blocking: false, steps: 1 },
            Operation::SLLI   => ExecutionLen { blocking: false, steps: 1 },
            Operation::SRLI   => ExecutionLen { blocking: false, steps: 1 },
            Operation::SRAI   => ExecutionLen { blocking: false, steps: 1 },
            Operation::ADD    => ExecutionLen { blocking: false, steps: 1 },
            Operation::SUB    => ExecutionLen { blocking: false, steps: 1 },
            Operation::SLL    => ExecutionLen { blocking: false, steps: 1 },
            Operation::SLT    => ExecutionLen { blocking: false, steps: 1 },
            Operation::SLTU   => ExecutionLen { blocking: false, steps: 1 },
            Operation::XOR    => ExecutionLen { blocking: false, steps: 1 },
            Operation::SRL    => ExecutionLen { blocking: false, steps: 1 },
            Operation::SRA    => ExecutionLen { blocking: false, steps: 1 },
            Operation::OR     => ExecutionLen { blocking: false, steps: 1 },
            Operation::AND    => ExecutionLen { blocking: false, steps: 1 },
            Operation::FENCE  => ExecutionLen { blocking: false, steps: 1 },
            Operation::FENCEI => ExecutionLen { blocking: false, steps: 1 },
            Operation::ECALL  => ExecutionLen { blocking: false, steps: 1 },
            Operation::EBREAK => ExecutionLen { blocking: false, steps: 1 },
            Operation::CSRRW  => ExecutionLen { blocking: false, steps: 1 },
            Operation::CSRRS  => ExecutionLen { blocking: false, steps: 1 },
            Operation::CSRRC  => ExecutionLen { blocking: false, steps: 1 },
            Operation::CSRRWI => ExecutionLen { blocking: false, steps: 1 },
            Operation::CSRRSI => ExecutionLen { blocking: false, steps: 1 },
            Operation::CSRRCI => ExecutionLen { blocking: false, steps: 1 },
            Operation::MUL    => ExecutionLen { blocking: false, steps: 3 },
            Operation::MULH   => ExecutionLen { blocking: false, steps: 3 },
            Operation::MULHSU => ExecutionLen { blocking: false, steps: 3 },
            Operation::MULHU  => ExecutionLen { blocking: false, steps: 3 },
            Operation::DIV    => ExecutionLen { blocking:  true, steps: 7 },
            Operation::DIVU   => ExecutionLen { blocking:  true, steps: 7 },
            Operation::REM    => ExecutionLen { blocking:  true, steps: 7 },
            Operation::REMU   => ExecutionLen { blocking:  true, steps: 7 },
        }
    }
}

impl From<Operation> for UnitType {
    #[rustfmt::skip]
    fn from(op: Operation) -> UnitType {
        match op {
            Operation::LUI    => UnitType::ALU,
            Operation::AUIPC  => UnitType::BLU,
            Operation::JAL    => UnitType::BLU,
            Operation::JALR   => UnitType::BLU,
            Operation::BEQ    => UnitType::BLU,
            Operation::BNE    => UnitType::BLU,
            Operation::BLT    => UnitType::BLU,
            Operation::BGE    => UnitType::BLU,
            Operation::BLTU   => UnitType::BLU,
            Operation::BGEU   => UnitType::BLU,
            Operation::LB     => UnitType::MCU,
            Operation::LH     => UnitType::MCU,
            Operation::LW     => UnitType::MCU,
            Operation::LBU    => UnitType::MCU,
            Operation::LHU    => UnitType::MCU,
            Operation::SB     => UnitType::MCU,
            Operation::SH     => UnitType::MCU,
            Operation::SW     => UnitType::MCU,
            Operation::ADDI   => UnitType::ALU,
            Operation::SLTI   => UnitType::ALU,
            Operation::SLTIU  => UnitType::ALU,
            Operation::XORI   => UnitType::ALU,
            Operation::ORI    => UnitType::ALU,
            Operation::ANDI   => UnitType::ALU,
            Operation::SLLI   => UnitType::ALU,
            Operation::SRLI   => UnitType::ALU,
            Operation::SRAI   => UnitType::ALU,
            Operation::ADD    => UnitType::ALU,
            Operation::SUB    => UnitType::ALU,
            Operation::SLL    => UnitType::ALU,
            Operation::SLT    => UnitType::ALU,
            Operation::SLTU   => UnitType::ALU,
            Operation::XOR    => UnitType::ALU,
            Operation::SRL    => UnitType::ALU,
            Operation::SRA    => UnitType::ALU,
            Operation::OR     => UnitType::ALU,
            Operation::AND    => UnitType::ALU,
            Operation::FENCE  => UnitType::MCU,
            Operation::FENCEI => UnitType::MCU,
            Operation::ECALL  => UnitType::MCU,
            Operation::EBREAK => UnitType::MCU,
            Operation::CSRRW  => UnitType::MCU,
            Operation::CSRRS  => UnitType::MCU,
            Operation::CSRRC  => UnitType::MCU,
            Operation::CSRRWI => UnitType::MCU,
            Operation::CSRRSI => UnitType::MCU,
            Operation::CSRRCI => UnitType::MCU,
            Operation::MUL    => UnitType::ALU,
            Operation::MULH   => UnitType::ALU,
            Operation::MULHSU => UnitType::ALU,
            Operation::MULHU  => UnitType::ALU,
            Operation::DIV    => UnitType::ALU,
            Operation::DIVU   => UnitType::ALU,
            Operation::REM    => UnitType::ALU,
            Operation::REMU   => UnitType::ALU,
        }
    }
}

impl ExecuteUnit {
    /// Constructs a new execute unit with given properties.
    pub fn new(unit_type: UnitType, pipeline_size: usize) -> ExecuteUnit {
        ExecuteUnit {
            unit_type,
            pipeline_size,
            executing: VecDeque::new(),
        }
    }

    /// Returns what type of execution unit this is.
    pub fn get_type(&self) -> UnitType {
        self.unit_type
    }

    /// Indicates whether or not this Execute Unit is free to take on another
    /// specified instruction.
    pub fn is_free(&self, el: ExecutionLen) -> bool {
        if el.blocking {
            return self.executing.is_empty();
        }
        // Note: Issue is run before the execute/writeback stage, so we need
        // to take into account that even if the pipeline is full, if the front
        // instruction is about to be popped off, the EU is actually free.
        match self.executing.front() {
            Some((_, len)) => {
                // Check if not blocking (in this case there should be only 1)
                !len.blocking && // AND
                // Either the pipeline is free, or is about to be free
                ((self.executing.len() < self.pipeline_size) ||
                 (len.steps == 1 && self.executing.len() <= self.pipeline_size))
            }
            None => true, // Nothing in the queue, so free
        }
    }

    /// Handles the logic for the execution of an
    /// [`Operation`](../../isa/op_code/enum.Operation.html) that this
    /// execution unit is responsible for. If the execute unit is pipelined,
    /// this will add the execution to the pipeline.
    pub fn handle_issue(&mut self, state_p: &State, reservation: &Reservation) {
        if self.unit_type != UnitType::from(reservation.op) {
            panic!(format!(
                "Execute Unit ({:?}) was given Operation ({:?}) that it is incapable of processing",
                self.unit_type, reservation.op
            ))
        }

        match Format::from(reservation.op) {
            Format::R => self.ex_r_type(reservation, &state_p.reorder_buffer),
            Format::I => self.ex_i_type(reservation, &state_p.reorder_buffer),
            Format::S => self.ex_s_type(reservation),
            Format::B => self.ex_b_type(reservation, &state_p.reorder_buffer),
            Format::U => self.ex_u_type(reservation),
            Format::J => self.ex_j_type(reservation),
        }
    }

    /// Triggers an exection step, only modifying the given new execution unit.
    /// Will advance the exectuion pipeline, writing any completed executions
    /// to the reorder buffer, also setting the finished bit.
    pub fn advance_pipeline(
        &self,
        new_eu: &mut ExecuteUnit,
        rob: &mut ReorderBuffer,
        rs: &mut ResvStation
    ) {
        // Ensure we do not minus 1 from an execution added to the new state in
        // the issue stage (which may have touched the execute unit already)
        let iter = new_eu.executing.iter_mut().take(self.executing.len());

        // Progress all executions in pipeline
        for (_, len) in iter {
            if len.steps > 0 {
                len.steps -= 1
            }
        }

        // If instruction has finished, write back to reorder buffer
        if let Some((_, el)) = new_eu.executing.front() {
            if el.steps == 0 {
                let result: ExecuteResult = new_eu.executing.pop_front().unwrap().0;
                rob[result.rob_entry].act_pc = result.pc;
                rob[result.rob_entry].act_rd = result.rd;
                rob[result.rob_entry].finished = true;

                match rob[result.rob_entry].op {
                    Operation::LB  |
                    Operation::LH  |
                    Operation::LW  |
                    Operation::LBU |
                    Operation::LHU |
                    Operation::SB  |
                    Operation::SH  |
                    Operation::SW  => (),
                    _ => {
                        // Bypass, let everyone that is waiting for this
                        // register know it's value. (Lower down values).
                        if let Some(rd) = result.rd {
                            rs.execute_bypass(result.rob_entry, rd);
                            rob.execute_bypass(result.rob_entry, rd);
                        }
                        // Finish with the dependencies that this was using.
                        // (Higher up values).
                        if let Right(name) = rob[result.rob_entry].rs1 {
                            rob[name].ref_count -= 1;
                            rob[result.rob_entry].rs1 = Left(0);
                        }
                        if let Right(name) = rob[result.rob_entry].rs2 {
                            rob[name].ref_count -= 1;
                            rob[result.rob_entry].rs2 = Left(0);
                        }
                    }
                }
            }
        }
    }

    /// Flushes the execute unit, this would happen when the pipeline is
    /// invalidated and needs to be restarted from scratch.
    pub fn flush(&mut self) {
        self.executing.clear()
    }

    /// Executes an R type instruction, putting the results in self.
    fn ex_r_type(&mut self, r: &Reservation, rob: &ReorderBuffer) {
        let rs1_s = match r.rs1 {
            Left(val) => val,
            Right(name) => rob[name]
                .act_rd
                .expect("Execute unit ({:?}) R-type expected rs1!"),
        };
        let rs2_s = match r.rs2 {
            Left(val) => val,
            Right(name) => rob[name]
                .act_rd
                .expect("Execute unit ({:?}) R-type expected rs2!"),
        };
        let rs1_u = rs1_s as u32;
        let rs2_u = rs2_s as u32;
        #[rustfmt::skip]
        let rd_val = match r.op {
            Operation::ADD    => rs1_s.overflowing_add(rs2_s).0,
            Operation::SUB    => rs1_s.overflowing_sub(rs2_s).0,
            Operation::SLL    => rs1_s << (rs2_s & 0b11111),
            Operation::SLT    => (rs1_s < rs2_s) as i32,
            Operation::SLTU   => (rs1_u < rs2_u) as i32,
            Operation::XOR    => rs1_s ^ rs2_s,
            Operation::SRL    => (rs1_u >> (rs2_u & 0b11111)) as i32,
            Operation::SRA    => rs1_s >> (rs2_s & 0b11111),
            Operation::OR     => rs1_s | rs2_s,
            Operation::AND    => rs1_s & rs2_s,
            Operation::MUL    => rs1_s.overflowing_mul(rs2_s).0,
            Operation::MULH   => ((i64::from(rs1_s) * i64::from(rs2_s)) >> 32) as i32,
            Operation::MULHU  => ((u64::from(rs1_u) * u64::from(rs2_u)) >> 32) as i32,
            Operation::MULHSU => ((i64::from(rs1_s) * i64::from(rs2_u)) >> 32) as i32,
            Operation::DIV    => match rs2_s {
                                     0  => -1i32,
                                     _  => match rs1_s.overflowing_div(rs2_s) {
                                         (_, true) => i32::min_value(),
                                         (v, _)    => v,
                                     },
                                 },
            Operation::DIVU   => match rs2_s {
                                     0  => i32::max_value(),
                                     _  => (rs1_u / rs2_u) as i32,
                                 },
            Operation::REM    => match rs2_s {
                                     0 => rs1_s,
                                     _ => match rs1_s.overflowing_div(rs2_s) {
                                         (_, true) => 0,
                                         (v, _)    => v,
                                     }
                                 },
            Operation::REMU   => match rs2_s {
                                     0 => rs1_s,
                                     _ => (rs1_u % rs2_u) as i32,
                                 },
            _ => panic!("Unknown R-type instruction failed to execute.")
        };

        self.executing.push_back((
            ExecuteResult {
                rob_entry: r.rob_entry,
                pc: r.pc as i32 + 4,
                rd: Some(rd_val),
            },
            ExecutionLen::from(r.op),
        ))
    }

    /// Executes an I type instruction, modifying the borrowed state.
    fn ex_i_type(&mut self, r: &Reservation, rob: &ReorderBuffer) {
        let rs1_s = match r.rs1 {
            Left(val) => val,
            Right(name) => rob[name]
                .act_rd
                .expect("Execute unit ({:?}) I-type expected rs1!"),
        };
        let rs1_u = rs1_s as u32;
        let imm_s = r.imm.expect("Execute unit I-type missing imm!");
        let imm_u = imm_s as u32;

        #[rustfmt::skip]
        let rd_val = match r.op {
            Operation::JALR   => Some(r.pc as i32 + 4),
            Operation::LB     => None, //
            Operation::LH     => None, //
            Operation::LW     => None, // All done in commit stage
            Operation::LBU    => None, //
            Operation::LHU    => None, //
            Operation::ADDI   => Some( rs1_s +  imm_s),
            Operation::SLTI   => Some((rs1_s <  imm_s) as i32),
            Operation::SLTIU  => Some((rs1_u <  imm_u) as i32),
            Operation::XORI   => Some( rs1_s ^  imm_s),
            Operation::ORI    => Some( rs1_s |  imm_s),
            Operation::ANDI   => Some( rs1_s &  imm_s),
            Operation::SLLI   => Some( rs1_s << imm_s),
            Operation::SRLI   => Some((rs1_u >> imm_u) as i32),
            Operation::SRAI   => Some( rs1_s >> (imm_s & 0b11111)),
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
            _ => panic!("Unknown I-type instruction failed to execute.")
        };

        let pc_val = if r.op == Operation::JALR {
            if rs1_s != -1 {
                (rs1_s + imm_s) & !0b1
            } else {
                -1
            }
        } else {
            r.pc as i32 + 4
        };

        self.executing.push_back((
            ExecuteResult {
                rob_entry: r.rob_entry,
                pc: pc_val,
                rd: rd_val,
            },
            ExecutionLen::from(r.op),
        ))
    }

    /// Executes an S type instruction, modifying the borrowed state.
    fn ex_s_type(&mut self, r: &Reservation) {
        match r.op {
            Operation::SB => (), //
            Operation::SH => (), // All done in commit stage
            Operation::SW => (), //
            _ => panic!("Unknown S-type instruction failed to execute."),
        };

        self.executing.push_back((
            ExecuteResult {
                rob_entry: r.rob_entry,
                pc: r.pc as i32 + 4,
                rd: None,
            },
            ExecutionLen::from(r.op),
        ))
    }

    /// Executes an B type instruction, modifying the borrowed state.
    fn ex_b_type(&mut self, r: &Reservation, rob: &ReorderBuffer) {
        let rs1_s = match r.rs1 {
            Left(val) => val,
            Right(name) => rob[name]
                .act_rd
                .expect("Execute unit ({:?}) B-type expected rs1!"),
        };
        let rs2_s = match r.rs2 {
            Left(val) => val,
            Right(name) => rob[name]
                .act_rd
                .expect("Execute unit ({:?}) B-type expected rs2!"),
        };
        let rs1_u = rs1_s as u32;
        let rs2_u = rs2_s as u32;
        let imm = r.imm.expect("Execute unit B-type missing imm!");

        #[rustfmt::skip]
        let pc_val = r.pc as i32 + match r.op {
            Operation::BEQ  => if rs1_s == rs2_s { imm } else { 4 },
            Operation::BNE  => if rs1_s != rs2_s { imm } else { 4 },
            Operation::BLT  => if rs1_s <  rs2_s { imm } else { 4 },
            Operation::BGE  => if rs1_s >= rs2_s { imm } else { 4 },
            Operation::BLTU => if rs1_u <  rs2_u { imm } else { 4 },
            Operation::BGEU => if rs1_u >= rs2_u { imm } else { 4 },
            _ => panic!("Unknown B-type instruction failed to execute.")
        };

        self.executing.push_back((
            ExecuteResult {
                rob_entry: r.rob_entry,
                pc: pc_val,
                rd: None,
            },
            ExecutionLen::from(r.op),
        ))
    }

    /// Executes an U type instruction, modifying the borrowed state.
    fn ex_u_type(&mut self, r: &Reservation) {
        let pc = r.pc as i32;
        let imm = r.imm.expect("Execute unit U-type missing imm!");

        let rd_val = match r.op {
            Operation::LUI => imm,
            Operation::AUIPC => pc + imm,
            _ => panic!("Unknown U-type instruction failed to execute."),
        };

        self.executing.push_back((
            ExecuteResult {
                rob_entry: r.rob_entry,
                pc: pc + 4,
                rd: Some(rd_val),
            },
            ExecutionLen::from(r.op),
        ))
    }

    /// Executes an J type instruction, modifying the borrowed state.
    fn ex_j_type(&mut self, r: &Reservation) {
        let imm = r.imm.expect("Execute unit J-type missing imm!");

        match r.op {
            Operation::JAL => {
                let old_pc = r.pc as i32;
                self.executing.push_back((
                    ExecuteResult {
                        rob_entry: r.rob_entry,
                        pc: old_pc + imm,
                        rd: Some(old_pc + 4),
                    },
                    ExecutionLen::from(r.op),
                ))
            }
            _ => panic!("Unknown J-type instruction failed to execute."),
        }
    }
}

impl Display for UnitType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.alternate() {
            match self {
                UnitType::ALU => f.pad("A"),
                UnitType::BLU => f.pad("B"),
                UnitType::MCU => f.pad("M"),
            }
        } else {
            f.pad(&format!("{:?}", self))
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Runs the _Execute & Writeback_ stage on every
/// [`ExecuteUnit`](../execute/struct.ExecuteUnit.html) in the given previous
/// [`State`](../state/struct.State.html), `state_p`, while putting the new
/// results in the current [`State`](../state/struct.State.html), `state`.
pub fn execute_and_writeback_stage(state_p: &State, state: &mut State) {
    let iter_p = state_p.execute_units.iter();
    let iter = state.execute_units.iter_mut();
    // Loop over both past and current execute units at the same time
    for (eu_p, mut eu) in iter_p.zip(iter) {
        eu_p.advance_pipeline(&mut eu, &mut state.reorder_buffer, &mut state.resv_station)
    }
}
