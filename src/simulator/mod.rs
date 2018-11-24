use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use std::thread;

use io::{IoEvent, IoThread, SimulatorEvent};
use isa::Instruction;
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
    let mut state = load_elf(&config);

    loop {
        // FETCH STAGE
        state.l_fetch = state.memory.read_i32(
            state.register[Register::PC as usize] as usize
        );

        // DECODE STAGE
        state.l_decode = match Instruction::decode(state.l_fetch.word) {
            Some(i) => i,
            None => { panic!("Failed to decode instruction.") },
        };
        // io.tx.send(IoEvent::UpdateInstruction(state.l_decode)).unwrap();

        if state.l_decode.is_ret() {
            io.tx.send(IoEvent::Finish).unwrap();
            break;
        }

        // EXECUTE STAGE
        instruction::exec(&mut state);
        io.tx.send(IoEvent::UpdateState(state.clone())).unwrap();
        thread::sleep(Duration::from_millis(50));

        // Handle IO thread events
        match io.rx.try_recv() {
            Ok(e) => match e {
                SimulatorEvent::Finish => break,
            },
            Err(TryRecvError::Disconnected) => Exit::IoThreadError.exit(
                Some("IO Thread missing, assumed dead.")
            ),
            _ => {},
        }
    }
    
    io.handle.join();
}
