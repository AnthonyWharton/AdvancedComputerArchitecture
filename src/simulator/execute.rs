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
            if self.executing.len() == 0 {
                return true
            } else {
                return false
            }
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
        state_p: &State,
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
        let rs1 = match r.rs1 {
            Left(val)   => val,
            Right(name) => rf.read_at_name(name)
                .expect("Exeute unit missing rs1!"),
        };
        let rs2 = match r.rs2 {
            Left(val)   => val,
            Right(name) => rf.read_at_name(name)
                .expect("Execute unit missing rs2!"),
        };
        let rd = match r.op {
            Operation::ADD    => rs1.overflowing_add(rs2).0,
            Operation::SUB    => rs1.overflowing_sub(rs2).0,
            Operation::SLL    => rs1 << (rs2 & 0b11111),
            Operation::SLT    => (rs1 < rs2) as i32,
            Operation::SLTU   => ((rs1 as u32) < (rs2 as u32)) as i32,
            Operation::XOR    => rs1 ^ rs2,
            Operation::SRL    => ((rs1 as u32) >> ((rs2 & 0b11111) as u32)) as i32,
            Operation::SRA    => rs1 >> (rs2 & 0b11111),
            Operation::OR     => rs1 | rs2,
            Operation::AND    => rs1 & rs2,
            Operation::MUL    => rs1.overflowing_mul(rs2).0,
            Operation::MULH   => (((rs1 as i64) * (rs2 as i64)) >> 32) as i32,
            Operation::MULHU  => (((rs1 as u64) * (rs2 as u64)) >> 32) as i32,
            Operation::MULHSU => (((rs1 as i64) * (rs2 as i64).abs()) >> 32) as i32,
            Operation::DIV    => match rs2 {
                                     0  => -1i32,
                                     _  => match rs1.overflowing_div(rs2) {
                                         (_, true) => i32::min_value(),
                                         (v, _)    => v,
                                     },
                                 },
            Operation::DIVU   => match rs2 {
                                     0  => i32::max_value(),
                                     _  => ((rs1 as u32) / (rs2 as u32)) as i32,
                                 },
            Operation::REM    => match rs2 {
                                     0 => rs1,
                                     _ => match rs1.overflowing_div(rs2) {
                                         (_, true) => 0,
                                         (v, _)    => v,
                                     }
                                 },
            Operation::REMU   => match rs2 {
                                     0 => rs1,
                                     _ => ((rs1 as u32) % (rs2 as u32)) as i32,
                                 },
            _ => panic!("Unknown R type instruction failed to execute.")
        };

        self.executing.push_back((
            ExecuteLatch {
                rob_entry: r.rob_entry,
                pc: rf.read_reg(Register::PC).unwrap() + 4,
                rd: Some(rd),
            },
            ExecutionLen::from(r.op)
        ))
    }





    ///////////////////////////////////////////////////////////////////////////
    ////////////// EVERYTHING BELOW THIS POINT NEEDS HEAVY REFACTORING AND/OR
    ////////////// RE-WRITING. IT IS ALL PLACEHOLDER FROM THE OLD FILE.







    // /// Executes an I type instruction, modifying the borrowed state.
    // fn ex_i_type(state: &mut State, instruction: Instruction) {
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

    // /// Executes an S type instruction, modifying the borrowed state.
    // fn ex_s_type(state: &mut State, instruction: Instruction) {
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

    // /// Executes an B type instruction, modifying the borrowed state.
    // fn ex_b_type(state: &mut State, instruction: Instruction) {
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

    // /// Executes an U type instruction, modifying the borrowed state.
    // fn ex_u_type(state: &mut State, instruction: Instruction) {
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

    // /// Executes an J type instruction, modifying the borrowed state.
    // fn ex_j_type(state: &mut State, instruction: Instruction) {
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

}

