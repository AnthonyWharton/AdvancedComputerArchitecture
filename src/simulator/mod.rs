use std::sync::mpsc::{SendError, TryRecvError};
use std::time::Duration;
use std::thread::sleep;

use io::{IoEvent, IoThread, SimulatorEvent};
use util::config::Config;
use util::loader::load_elf;
use util::exit::Exit;
use self::state::State;

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

pub fn run_simulator(config: Config) {
    let memory = load_elf(&config);
    let _state  = State {
        register: vec!(0i32, 33),
        memory,
    };

    let io: IoThread = IoThread::new();

    let mut count = 0;
    loop {
        // Simulation
        if let Err(SendError(_)) = io.tx.send(IoEvent::DoneThing) { break; }
        count += 1;
        if count > 20 {
            io.tx.send(IoEvent::Exit).is_ok();
            break;
        }
        sleep(Duration::from_millis(500));

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
