use isa::op_code::Operation;
use simulator::reorder::ReorderEntry;
use simulator::reservation::Reservation;
use simulator::state::State;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// An enumeration of the different types of execute units that exist within
/// the simulator.
#[derive(PartialEq)]
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

/// An `ExecuteUnit` will provide functions that can be run to execute an
/// instruction in the execute stage, as well as deal with the results in the
/// writeback stage.
#[derive(Clone)]
pub struct ExecuteUnit {
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

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
        unimplemented!()
    }

    /// Indicates whether or not this Execute Unit is pipelined or not.
    pub fn is_pipelined(&self) -> bool {
        unimplemented!()
    }

    /// Indicates whether or not this Execute Unit is free to take on another
    /// instruction.
    pub fn is_free(&self) -> bool {
        unimplemented!()
    }

    /// Handles the logic for the execution of an
    /// [`Operation`](../../isa/op_code/enum.Operation.html) that this execution
    /// unit is responsible for. If the execute unit is pipelined, this will
    /// add the execution to the pipeline.
    pub fn handle_execute(
        &mut self,
        state_p: &State,
        state_n: &mut State,
        reservation: &Reservation,
    ) {
        unimplemented!()
    }

    /// Retrieves the results of the finished execute stage ready for the
    /// reorder buffer, if anything exists in the latch.
    pub fn get_result_latch(&mut self) -> Option<ReorderEntry> {
        unimplemented!()
    }

    /// Handles the logic for the writeback of an
    /// [`Operation`](../../isa/op_code/enum.Operation.html) that this execution
    /// unit is responsible for.
    pub fn handle_writeback(
        &mut self,
        state_p: &State,
        state_n: &mut State,
        rob_entry: &ReorderEntry,
    ) {
        unimplemented!()
    }
}

