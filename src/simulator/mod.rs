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
//// CONST/STATIC

pub const INITIALLY_PAUSED: bool = true;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Main entry point for the simulation.
///
/// Requires an IoThread for sending events to be output to the display, as
/// well as for receiving any calls to close the simulation.
pub fn run_simulator(io: IoThread, config: Config) {
    let mut state = load_elf(&config);
    let mut paused = INITIALLY_PAUSED;

    while handle_io_and_continue(&mut paused, &io) {
        // FETCH STAGE
        state.l_fetch = state.memory.read_i32(
            state.register[Register::PC as usize] as usize
        );

        // DECODE STAGE
        state.l_decode = match Instruction::decode(state.l_fetch.word) {
            Some(i) => i,
            None => { panic!("Failed to decode instruction.") },
        };

        // Check for return instruction, immediately shutdown simulator.
        if state.l_decode.is_ret() {
            io.tx.send(IoEvent::Finish).unwrap();
            break;
        }

        // EXECUTE STAGE
        instruction::exec(&mut state);

        state.stats.cycles += 1;

        // Update the IO thread.
        io.tx.send(IoEvent::UpdateState(state.clone())).unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    #[allow(unused_must_use)]
    { io.handle.join(); }
}

/// Handles any messages from the input/output thread. Will block if paused, &
/// not block if unpaused. Returns false when the user closed the simulator.
fn handle_io_and_continue(paused: &mut bool, io: &IoThread) -> bool {
    if *paused {
        loop {
            match io.rx.recv() {
                Ok(e) => return handle_message(e, paused),
                Err(_) => Exit::IoThreadError.exit(
                    Some("IO Thread stopped communication properly.")
                ),
            };
        }
    } else {
        match io.rx.try_recv() {
            Ok(e) => return handle_message(e, paused),
            Err(TryRecvError::Disconnected) => Exit::IoThreadError.exit(
                Some("IO Thread missing, assumed dead.")
            ),
            _ => return true,
        }
    }
}

/// Handles any messages from the input/output thread.
/// Returns false when the user closed the simulator.
fn handle_message(event: SimulatorEvent, paused: &mut bool) -> bool {
    match event {
        SimulatorEvent::Finish => return false,
        SimulatorEvent::PauseToggle => {
            *paused ^= true;
            return true
        },
        SimulatorEvent::Cycle => return true,
    }
}

