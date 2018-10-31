use std::sync::mpsc;
use std::thread;

use simulator::state::State;
use self::output::display_thread;

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// User Input event handler logic.
pub mod input;

/// Output display logic.
pub mod output;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// Events destined for the IO thread.
#[allow(dead_code)]
pub enum IoEvent {
    Exit,
    DoneThing,
    UpdateState(State),
}

/// Events destined for the simulator main thread.
pub enum SimulatorEvent {
    Exit,
}

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Wrapper class around the IO thread, which deals with user input and output.
///
///  - The `tx` field can be used to send `IoEvent`'s to the IO Thread.
///  - The `rx` field should be used with `try_recv` to receive 
///    `SimulatorEvent`'s.
pub struct IoThread {
    pub tx: mpsc::Sender<IoEvent>,
    pub rx: mpsc::Receiver<SimulatorEvent>,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl IoThread {
    pub fn new() -> IoThread {
        let (tx_m, rx_m) = mpsc::channel(); // Channel from io to MAIN
        let (tx_i, rx_i) = mpsc::channel(); // Channel from main to IO
        thread::spawn(|| display_thread(tx_m, rx_i));
        IoThread {
            tx: tx_i,
            rx: rx_m,
        }
    }
}

