use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use crate::io::{IoEvent, IoThread, SimulatorEvent};
use crate::util::config::Config;

use self::commit::commit_stage;
use self::decode::decode_and_rename_stage;
use self::issue::issue_stage;
use self::execute::execute_and_writeback_stage;
use self::fetch::fetch_stage;
use self::state::State;

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// Logic and data structures regarding the _fetch_ state of the pipeline. This
/// is responsible for coordinating the retrieval (or fetching) of instructions
/// from memory.
pub mod fetch;

/// Logic regarding the _decode/rename_ stage of the pipeline. This is
/// responsible for decoding instructions and ensuring they have no
/// dependencies when moving down the pipeline,
pub mod decode;

/// Logic regarding the _issue_ stage in the pipeline. This is responsible
/// for consuming 'ready' instructions from the reservation station, and
/// assigning them to execute units for the future _execute_ stage.
pub mod issue;

/// All of the execute units, which are responsible for the _execute_ stage in
/// the pipeline, as well as the logic for what should happen at writeback for
/// a particular instruction.
pub mod execute;

/// Logic recarding the _commit_ stage in the pipeline. This is responsible
/// for committing the results of instructions that have finished execution.
pub mod commit;

/// Locic and datastructures for the branch predictor, used to inform the
/// _fetch_ stage of which instruction to fetch next for speculative execution.
pub mod branch;

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
pub fn run_simulator(io: IoThread, config: &Config) {
    let mut state = State::new(&config);
    let mut paused = INITIALLY_PAUSED;

    // Send the initial state to the UI to be displayed
    io.tx.send(IoEvent::UpdateState(state.clone())).unwrap();

    while handle_io_and_continue(&mut paused, &io) {
        // Maintain immutable past state
        let state_p = state.clone();

        fetch_stage(&state_p, &mut state);
        decode_and_rename_stage(&state_p, &mut state);
        issue_stage(&state_p, &mut state);
        execute_and_writeback_stage(&state_p, &mut state);
        let finished = commit_stage(&state_p, &mut state);

        // End of cycle, start housekeeping
        state.stats.cycles += 1;

        // Update IO thread and sleep for a moment
        io.tx.send(IoEvent::UpdateState(state.clone())).unwrap();
        if finished {
            io.tx.send(IoEvent::Finish).unwrap();
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }

    #[allow(unused_must_use)]
    {
        io.handle.join();
    }
}

/// Handles any messages from the input/output thread. Will block if paused, &
/// not block if unpaused. Returns false when the user closed the simulator.
fn handle_io_and_continue(paused: &mut bool, io: &IoThread) -> bool {
    if *paused {
        loop {
            match io.rx.recv() {
                Ok(e) => return handle_message(e, paused),
                Err(_) => error!("IO Thread stopped communication properly."),
            };
        }
    } else {
        match io.rx.try_recv() {
            Ok(e) => handle_message(e, paused),
            Err(TryRecvError::Disconnected) => error!("IO Thread missing, assumed dead."),
            _ => true,
        }
    }
}

/// Handles any messages from the input/output thread.
/// Returns false when the user closed the simulator.
fn handle_message(event: SimulatorEvent, paused: &mut bool) -> bool {
    match event {
        SimulatorEvent::Finish => false,
        SimulatorEvent::PauseToggle => {
            *paused ^= true;
            true
        }
        SimulatorEvent::Cycle => true,
    }
}
