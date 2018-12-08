use std::collections::VecDeque;

use either::{Left, Right};

use isa::Format;
use isa::op_code::Operation;
use isa::operand::Register;
use simulator::register::RegisterFile;
use simulator::reservation::Reservation;
use simulator::state::State;

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
    unit_type: UnitType,
    /// The depth of the pipeline size for this execute unit. A value of 1 is
    /// a non-pipelined unit.
    pipeline_size: usize,
    /// The pipeline of executing instructions, and how many cycles left in the
    /// execution of the instruction.
    executing: VecDeque<(ExecuteLatch, ExecutionLen)>,
}

/// The latch that contains the resulting information from the execute unit
/// upon completion.
#[derive(Copy, Clone, Debug)]
pub struct ExecuteLatch {
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
            Operation::LB     => ExecutionLen { blocking:  true, steps: 1 },
            Operation::LH     => ExecutionLen { blocking:  true, steps: 1 },
            Operation::LW     => ExecutionLen { blocking:  true, steps: 1 },
            Operation::LBU    => ExecutionLen { blocking:  true, steps: 1 },
            Operation::LHU    => ExecutionLen { blocking:  true, steps: 1 },
            Operation::SB     => ExecutionLen { blocking:  true, steps: 1 },
            Operation::SH     => ExecutionLen { blocking:  true, steps: 1 },
            Operation::SW     => ExecutionLen { blocking:  true, steps: 1 },
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
            Operation::MUL    => ExecutionLen { blocking: false, steps: 1 },
            Operation::MULH   => ExecutionLen { blocking: false, steps: 1 },
            Operation::MULHSU => ExecutionLen { blocking: false, steps: 1 },
            Operation::MULHU  => ExecutionLen { blocking: false, steps: 1 },
            Operation::DIV    => ExecutionLen { blocking:  true, steps: 1 },
            Operation::DIVU   => ExecutionLen { blocking:  true, steps: 1 },
            Operation::REM    => ExecutionLen { blocking:  true, steps: 1 },
            Operation::REMU   => ExecutionLen { blocking:  true, steps: 1 },
        }
    }
}

impl From<Operation> for UnitType {
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
    /// Returns what type of execution unit this is.
    pub fn get_type(&self) -> UnitType {
        self.unit_type
    }

    /// Indicates whether or not this Execute Unit is pipelined or not.
    pub fn is_pipelined(&self) -> bool {
        self.pipeline_size > 1
    }

    /// Indicates whether or not this Execute Unit is free to take on another
    /// specified instruction.
    pub fn is_free(&self, op: Operation) -> bool {
        if ExecutionLen::from(op).blocking {
            return self.executing.is_empty()
        }
        // Note: Dispatch is run before the execute/writeback stage, so we need
        // to take into account that even if the pipeline is full, if the front
        // instruction is about to be popped off, the EU is actually free.
        match self.executing.front() {
            Some((_, len)) => {
                // Check if not blocking (in this case there should be only 1)
                !len.blocking && // AND
                // Either the pipeline is free, or is about to be free
                ((self.executing.len() < self.pipeline_size) ||
                 (len.steps == 1 && self.executing.len() <= self.pipeline_size))
            },
            None => true, // Nothing in the queue, so free
        }
    }

    /// Handles the logic for the execution of an
    /// [`Operation`](../../isa/op_code/enum.Operation.html) that this execution
    /// unit is responsible for. If the execute unit is pipelined, this will
    /// add the execution to the pipeline.
    pub fn handle_execute(
        &mut self,
        _state_p: &State,
        reservation: &Reservation,
    ) {
        if self.unit_type != UnitType::from(reservation.op) {
            panic!(format!(
                "Execute Unit {:?} was given Operation {:?}, which it didn't \
                know how to calculate", self.unit_type, reservation.op)
            )
        }

        match Format::from(reservation.op) {
            Format::R => unimplemented!(), //self.ex_r_type(&state_p.register, reservation),
            Format::I => unimplemented!(),
            Format::S => unimplemented!(),
            Format::B => unimplemented!(),
            Format::U => unimplemented!(),
            Format::J => unimplemented!(),
        }
    }

    /// Triggers another execution step, advancing the pipeline of executing
    /// instructions. If an instruction has finished passing the pipeline, it
    /// will be returned by this function.
    pub fn advance_pipeline(&mut self) -> Option<ExecuteLatch> {
        self.executing.iter_mut().for_each(|(_, len)| len.steps -= 1);
        match self.executing.pop_front() {
            Some((el, len)) if len.steps == 0 => Some(el),
            Some(entry) => {
                self.executing.push_front(entry);
                None
            },
            _ => None,
        }
    }

    /// Executes an R type instruction, putting the results in self.
    fn ex_r_type(&mut self, rf: &RegisterFile, r: &Reservation) {
        let rs1s = match r.rs1 {
            Left(val)   => val,
            Right(name) => rf.read_at_name(name)
                .expect("Execute unit missing rs1!"),
        };
        let rs2s = match r.rs2 {
            Left(val)   => val,
            Right(name) => rf.read_at_name(name)
                .expect("Execute unit missing rs2!"),
        };
        let rs1u = rs1s as u32;
        let rs2u = rs2s as u32;
        let rd_val = match r.op {
            Operation::ADD    => rs1s.overflowing_add(rs2s).0,
            Operation::SUB    => rs1s.overflowing_sub(rs2s).0,
            Operation::SLL    => rs1s << (rs2s & 0b11111),
            Operation::SLT    => (rs1s < rs2s) as i32,
            Operation::SLTU   => (rs1u < rs2u) as i32,
            Operation::XOR    => rs1s ^ rs2s,
            Operation::SRL    => (rs1u >> (rs2u & 0b11111)) as i32,
            Operation::SRA    => rs1s >> (rs2s & 0b11111),
            Operation::OR     => rs1s | rs2s,
            Operation::AND    => rs1s & rs2s,
            Operation::MUL    => rs1s.overflowing_mul(rs2s).0,
            Operation::MULH   => ((i64::from(rs1s) * i64::from(rs2s)) >> 32) as i32,
            Operation::MULHU  => ((u64::from(rs1u) * u64::from(rs2u)) >> 32) as i32,
            Operation::MULHSU => ((i64::from(rs1s) * i64::from(rs2u)) >> 32) as i32,
            Operation::DIV    => match rs2s {
                                     0  => -1i32,
                                     _  => match rs1s.overflowing_div(rs2s) {
                                         (_, true) => i32::min_value(),
                                         (v, _)    => v,
                                     },
                                 },
            Operation::DIVU   => match rs2s {
                                     0  => i32::max_value(),
                                     _  => (rs1u / rs2u) as i32,
                                 },
            Operation::REM    => match rs2s {
                                     0 => rs1s,
                                     _ => match rs1s.overflowing_div(rs2s) {
                                         (_, true) => 0,
                                         (v, _)    => v,
                                     }
                                 },
            Operation::REMU   => match rs2s {
                                     0 => rs1s,
                                     _ => (rs1u % rs2u) as i32,
                                 },
            _ => panic!("Unknown R type instruction failed to execute.")
        };

        self.executing.push_back((
            ExecuteLatch {
                rob_entry: r.rob_entry,
                pc: rf.read_reg(Register::PC).unwrap() + 4,
                rd: Some(rd_val),
            },
            ExecutionLen::from(r.op)
        ))
    }

    /// Executes an I type instruction, modifying the borrowed state.
    fn ex_i_type(&mut self, rf: &RegisterFile, r: &Reservation) {
        let rs1 = match r.rs1 {
            Left(val)   => val,
            Right(name) => rf.read_at_name(name)
                .expect("Execute unit missing rs1!"),
        };
        let imm = r.imm.expect("Execute unit missing imm!");

        let rd_val = match r.op {
            Operation::JALR   => Some(rf.read_reg(Register::PC).unwrap() + 4),
            // TODO Move to writeback stage
            // Operation::LB     => m[(rs1 + imm) as usize] as i8 as i32,
            // Operation::LH     => m.read_i16((rs1 + imm) as usize).word as i32,
            // Operation::LW     => m.read_i32((rs1 + imm) as usize).word,
            // Operation::LBU    => m[(rs1 + imm) as usize] as i32,
            // Operation::LHU    => m.read_u16((rs1 + imm) as usize).word as i32,
            Operation::LB     => None,
            Operation::LH     => None,
            Operation::LW     => None,
            Operation::LBU    => None,
            Operation::LHU    => None,
            Operation::ADDI   => Some(rs1 + imm),
            Operation::SLTI   => Some((rs1 < imm) as i32),
            Operation::SLTIU  => Some(((rs1 as u32) < (imm as u32)) as i32),
            Operation::XORI   => Some(rs1 ^ imm),
            Operation::ORI    => Some(rs1 | imm),
            Operation::ANDI   => Some(rs1 & imm),
            Operation::SLLI   => Some(rs1 << imm),
            Operation::SRLI   => Some(((rs1 as u32) >> (imm as u32)) as i32),
            Operation::SRAI   => Some(rs1 >> (imm & 0b11111)),
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
            _ => panic!("Unknown I type instruction failed to execute.")
        };

        let pc_val = if r.op == Operation::JALR {
            (rs1 + imm) & !0b1
        } else {
            rf.read_reg(Register::PC).unwrap() + 4
        };

        self.executing.push_back((
            ExecuteLatch {
                rob_entry: r.rob_entry,
                pc: pc_val,
                rd: rd_val,
            },
            ExecutionLen::from(r.op)
        ))
    }

    /// Executes an S type instruction, modifying the borrowed state.
    fn ex_s_type(&mut self, rf: &RegisterFile, r: &Reservation) {
        // let rs1 = match r.rs1 {
        //     Left(val)   => val,
        //     Right(name) => rf.read_at_name(name)
        //         .expect("Execute unit missing rs1!"),
        // };
        // let rs2 = match r.rs2 {
        //     Left(val)   => val,
        //     Right(name) => rf.read_at_name(name)
        //         .expect("Execute unit missing rs2!"),
        // };
        // let imm = r.imm.expect("Execute unit missing imm!");

        match r.op {
            // TODO Move to writeback stage
            // Operation::SB => { m[(rs1 + imm) as usize] = rs2 as u8 },
            // Operation::SH => { m.write_i16((rs1 + imm) as usize, rs2 as i16); () },
            // Operation::SW => { m.write_i32((rs1 + imm) as usize, rs2); () },
            Operation::SB => (),
            Operation::SH => (),
            Operation::SW => (),
            _ => panic!("Unknown s type instruction failed to execute.")
        };

        self.executing.push_back((
            ExecuteLatch {
                rob_entry: r.rob_entry,
                pc: rf.read_reg(Register::PC).unwrap() + 4,
                rd: None,
            },
            ExecutionLen::from(r.op)
        ))
    }

    /// Executes an B type instruction, modifying the borrowed state.
    fn ex_b_type(&mut self, rf: &RegisterFile, r: &Reservation) {
        let rs1 = match r.rs1 {
            Left(val)   => val,
            Right(name) => rf.read_at_name(name)
                .expect("Execute unit missing rs1!"),
        };
        let rs2 = match r.rs2 {
            Left(val)   => val,
            Right(name) => rf.read_at_name(name)
                .expect("Execute unit missing rs2!"),
        };
        let imm = r.imm.expect("Execute unit missing imm!");

        let pc_val = rf.read_reg(Register::PC).unwrap() + match r.op {
            Operation::BEQ  => if rs1 == rs2 { imm } else { 4 },
            Operation::BNE  => if rs1 != rs2 { imm } else { 4 },
            Operation::BLT  => if rs1 <  rs2 { imm } else { 4 },
            Operation::BGE  => if rs1 >= rs2 { imm } else { 4 },
            Operation::BLTU =>
                if (rs1 as u32) <  (rs2 as u32) { imm } else { 4 },
            Operation::BGEU =>
                if (rs1 as u32) >= (rs2 as u32) { imm } else { 4 },
            _ => panic!("Unknown B type instruction failed to execute.")
        };

        self.executing.push_back((
            ExecuteLatch {
                rob_entry: r.rob_entry,
                pc: pc_val,
                rd: None,
            },
            ExecutionLen::from(r.op)
        ))
    }

    /// Executes an U type instruction, modifying the borrowed state.
    fn ex_u_type(&mut self, rf: &RegisterFile, r: &Reservation) {
        let imm = r.imm.expect("Execute unit missing imm!");

        let rd_val = match r.op {
            Operation::LUI   => Some(imm),
            Operation::AUIPC => None,
            _ => panic!("Unknown U type instruction failed to execute.")
        };

        let pc_val = rf.read_reg(Register::PC).unwrap() + match r.op {
            Operation::LUI   => 4,
            Operation::AUIPC => imm,
            _ => panic!("Unknown U type instruction failed to execute.")
        };

        self.executing.push_back((
            ExecuteLatch {
                rob_entry: r.rob_entry,
                pc: pc_val,
                rd: rd_val,
            },
            ExecutionLen::from(r.op)
        ))
    }

    /// Executes an J type instruction, modifying the borrowed state.
    fn ex_j_type(&mut self, rf: &RegisterFile, r: &Reservation) {
        let imm = r.imm.expect("Execute unit missing imm!");

        match r.op {
            Operation::JALR => {
                let old_pc = rf.read_reg(Register::PC).unwrap();
                self.executing.push_back((
                    ExecuteLatch {
                        rob_entry: r.rob_entry,
                        pc: old_pc + imm,
                        rd: Some(old_pc + 4),
                    },
                    ExecutionLen::from(r.op)
                ))
            },
            _ => panic!("Unknown J type instruction failed to execute."),
        }
    }
}

