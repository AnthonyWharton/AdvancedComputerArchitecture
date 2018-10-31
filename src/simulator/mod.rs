use std::sync::mpsc::{SendError, TryRecvError};
use std::thread;

use io::{IoEvent, IoThread, SimulatorEvent};
use isa::{Instruction, Word};
use isa::operand::Register;
use util::config::Config;
use util::loader::load_elf;
use util::exit::Exit;

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// Definitions for the execution of every function.
mod instruction;

/// Definitions for the main memory data structure.
pub mod memory;

/// Definitions for the ongoing state of the simulator.
pub mod state;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Main entry point for the simulation.
///
/// Requires an IoThread for sending events to be output to the display, as
/// well as for receiving any calls to close the simulation.
pub fn run_simulator(io: IoThread, config: Config) {
    let r = load_elf(&config);
    let mut state  = r.0;
    let mut memory = r.1;

    let mut inst_raw: Word = 0;
    let mut inst: Instruction = Instruction::default();
    let mut _aligned = true;
    loop {
        // FETCH STAGE
        let r = memory.read_word(state.register[Register::PC as usize] as usize);
        inst_raw = r.0;
        _aligned = r.1;

        // DECODE STAGE
        inst = match Instruction::decode(inst_raw) {
            Some(i) => i,
            None => { panic!("Failed to decode instruction.") },
        };

        // EXECUTE STAGE
        instruction::exec(&inst, &state, &memory);
        io.tx.send(IoEvent::UpdateState(state)).unwrap();
        thread::sleep_ms(1000);

        // Handle IO thread events
        match io.rx.try_recv(){
            Ok(e) => match e {
                SimulatorEvent::Exit => break,
            },
            Err(TryRecvError::Disconnected) => Exit::IoThreadError.exit(
                Some("IO Thread missing, assumed dead.")
            ),
            _ => {},
        }
    }
}
