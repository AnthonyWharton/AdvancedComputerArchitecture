use isa::op_code::Operation;

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// Logic that is used by the _Arithmetic Logic Unit_.
pub mod alu;

/// Logic that is used by the _Branch Logic Unit_.
pub mod blu;

/// Logic that is used by the _Memory & Control Unit_.
pub mod mcu;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// An enumeration of the different types of execute units that exist within
/// the simulator.
pub enum ExecuteUnit {
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
//// IMPLEMENTATIONS

impl From<Operation> for ExecuteUnit {
    fn from(op: Operation) -> ExecuteUnit {
        match op {
            Operation::LUI    => ExecuteUnit::ALU,
            Operation::AUIPC  => ExecuteUnit::BLU,
            Operation::JAL    => ExecuteUnit::BLU,
            Operation::JALR   => ExecuteUnit::BLU,
            Operation::BEQ    => ExecuteUnit::BLU,
            Operation::BNE    => ExecuteUnit::BLU,
            Operation::BLT    => ExecuteUnit::BLU,
            Operation::BGE    => ExecuteUnit::BLU,
            Operation::BLTU   => ExecuteUnit::BLU,
            Operation::BGEU   => ExecuteUnit::BLU,
            Operation::LB     => ExecuteUnit::MCU,
            Operation::LH     => ExecuteUnit::MCU,
            Operation::LW     => ExecuteUnit::MCU,
            Operation::LBU    => ExecuteUnit::MCU,
            Operation::LHU    => ExecuteUnit::MCU,
            Operation::SB     => ExecuteUnit::MCU,
            Operation::SH     => ExecuteUnit::MCU,
            Operation::SW     => ExecuteUnit::MCU,
            Operation::ADDI   => ExecuteUnit::ALU,
            Operation::SLTI   => ExecuteUnit::ALU,
            Operation::SLTIU  => ExecuteUnit::ALU,
            Operation::XORI   => ExecuteUnit::ALU,
            Operation::ORI    => ExecuteUnit::ALU,
            Operation::ANDI   => ExecuteUnit::ALU,
            Operation::SLLI   => ExecuteUnit::ALU,
            Operation::SRLI   => ExecuteUnit::ALU,
            Operation::SRAI   => ExecuteUnit::ALU,
            Operation::ADD    => ExecuteUnit::ALU,
            Operation::SUB    => ExecuteUnit::ALU,
            Operation::SLL    => ExecuteUnit::ALU,
            Operation::SLT    => ExecuteUnit::ALU,
            Operation::SLTU   => ExecuteUnit::ALU,
            Operation::XOR    => ExecuteUnit::ALU,
            Operation::SRL    => ExecuteUnit::ALU,
            Operation::SRA    => ExecuteUnit::ALU,
            Operation::OR     => ExecuteUnit::ALU,
            Operation::AND    => ExecuteUnit::ALU,
            Operation::FENCE  => ExecuteUnit::MCU,
            Operation::FENCEI => ExecuteUnit::MCU,
            Operation::ECALL  => ExecuteUnit::MCU,
            Operation::EBREAK => ExecuteUnit::MCU,
            Operation::CSRRW  => ExecuteUnit::MCU,
            Operation::CSRRS  => ExecuteUnit::MCU,
            Operation::CSRRC  => ExecuteUnit::MCU,
            Operation::CSRRWI => ExecuteUnit::MCU,
            Operation::CSRRSI => ExecuteUnit::MCU,
            Operation::CSRRCI => ExecuteUnit::MCU,
            Operation::MUL    => ExecuteUnit::ALU,
            Operation::MULH   => ExecuteUnit::ALU,
            Operation::MULHSU => ExecuteUnit::ALU,
            Operation::MULHU  => ExecuteUnit::ALU,
            Operation::DIV    => ExecuteUnit::ALU,
            Operation::DIVU   => ExecuteUnit::ALU,
            Operation::REM    => ExecuteUnit::ALU,
            Operation::REMU   => ExecuteUnit::ALU,
        }
    }
}

