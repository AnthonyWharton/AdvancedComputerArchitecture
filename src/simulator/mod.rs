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
    let r = load_elf(&config);
    let mut state  = r.0;
    let mut memory = r.1;

    loop {
        // FETCH STAGE
        let r = memory.read_i32(state.register[Register::PC as usize] as usize);
        let inst_raw = r.0;
        let _aligned = r.1;

        // DECODE STAGE
        let inst = match Instruction::decode(inst_raw) {
            Some(i) => i,
            None => { panic!("Failed to decode instruction.") },
        };
        io.tx.send(IoEvent::UpdateInstruction(inst)).unwrap();

        if inst.is_ret() {
            io.tx.send(IoEvent::Exit).unwrap();
            break;
        }

        // EXECUTE STAGE
        instruction::exec(&inst, &mut state, &mut memory);
        io.tx.send(IoEvent::UpdateState(state)).unwrap();
        thread::sleep(Duration::from_millis(50));

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
