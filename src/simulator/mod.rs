use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use std::thread;

use io::{IoEvent, IoThread, SimulatorEvent};
use util::config::Config;
use util::loader::load_elf;
use util::exit::Exit;

use self::decode::decode_and_rename_stage;
use self::fetch::fetch_stage;

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// Logic and data structures regarding the fetch state of the pipeline. This
/// is responsible for coordinating the retrieval (or fetching) of instructions
/// from memory.
mod fetch;

/// Logic regarding the decode and rename section of the pipeline. This is
/// responsible for decoding instructions and ensuring they have no
/// dependencies when moving down the pipeline,
mod decode;

/// All of the execute units, which are responsible for the execute stage in
/// the pipeline, as well as the logic for what should happen at writeback for
/// a particular instruction.
mod execute;

/// Logic recarding the writeback section of the pipeline. This is responsible
/// for committing the results of instructions that have finished execution.
pub mod writeback;

/// _To be replaced._
///
/// Definitions for the execution of every function.
mod instruction;

/// Logic and data structures for the simulated main memory module, which is
/// where program instructions and data are stored.
pub mod memory;

/// Logic and data structures for the architectural and physical register
/// files, which are where temporary working values used by the simulated
/// processor are stored.
pub mod register;

/// Logic and data structures for the reorder buffer, which is used to ensure
/// that instructions executed out of order will be committed in the order of
/// execution.
pub mod reorder;

/// Logic and data structures for the unified reservation station, which is
/// responsible for holding decoded instructions that are pending execution.
pub mod reservation;

/// Definitions for the ongoing state of the simulator. This encapsulates
/// almost all of the submodules within this module.
pub mod state;


///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

/// Whether or not the simulator is initially paused upon being opened.
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
        // Maintain immutable past state
        let state_p = state.clone();

        fetch_stage(&state_p, &mut state);
        decode_and_rename_stage(&state_p, &mut state);

        // EXECUTE STAGE
        if let Some(instruction) = state_p.l_decode {
            if instruction.is_ret() {
                io.tx.send(IoEvent::Finish).unwrap();
                break;
            }
            instruction::exec(&mut state, instruction);
            state.l_decode = None;
        }

        // End of cycle, start housekeeping
        state.stats.cycles += 1;

        // Update IO thread and sleep for a moment
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

