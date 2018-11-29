extern crate tui;
extern crate termion;

use std::cmp;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread::{JoinHandle, spawn};

use self::termion::event::Key;
use self::tui::layout::Rect;

use simulator::INITIALLY_PAUSED;
use simulator::state::State;
use util::exit::Exit;
use self::input::spawn_input_thread;
use self::output::{draw_state, new_terminal};

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// User Input event handler logic.
pub mod input;

/// Output display logic.
pub mod output;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

/// The number of states to keep in memory.
/// Each state uses approximately O(sim_mem_size) RAM, which is typically 1mb.
pub const KEPT_STATES: usize = 100;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// Events destined for the IO thread.
#[allow(dead_code)]
pub enum IoEvent {
    /// Signal that the user has asked to exit the process.
    Exit,
    /// Signal that the simulation has finished.
    Finish,
    /// Signal that a keypress has occured (from the input thread).
    Input(Key),
    /// Signal that the state has updated after a clock cycle.
    UpdateState(State),
}

/// Events destined for the simulator main thread.
pub enum SimulatorEvent {
    /// Signal that the user has asked to stop the simulator.
    Finish,
    /// Signal that the the simulation pause state should be toggled.
    PauseToggle,
    /// Signal that (when paused) the processor should execute one clock cycle.
    Cycle,
}

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Wrapper class around the IO thread, which deals with user input and output.
///
///  - The `tx` field can be used to send `IoEvent`'s to the IO Thread.
///  - The `rx` field should be used with `try_recv` to receive
///    `SimulatorEvent`'s.
pub struct IoThread {
    pub tx: Sender<IoEvent>,
    pub rx: Receiver<SimulatorEvent>,
    pub handle: JoinHandle<()>
}

/// Encapsulation of the state for the TuiApp front-end.
pub struct TuiApp {
    /// Sender to send messages to the simulator
    pub tx: Sender<SimulatorEvent>,
    /// Reciever for events send from the simulator
    pub rx: Receiver<IoEvent>,
    /// Terminal size
    pub size: Rect,
    /// History of the last KEPT_STATES states
    pub states: VecDeque<State>,
    /// Whether or not the simulator has finished
    pub finished: bool,
    /// Whether or not the simulator is paused
    pub paused: bool,
    /// Which historical state we are showing.
    /// 0 is current, 1 is the state before, 2 is the state before 1, etc
    pub hist_display: usize,
}


///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl IoThread {
    /// Creates a new IoThread object, and spawns the input/out threads
    /// to run in the background.
    pub fn new() -> IoThread {
        let (tx_m, rx_m) = channel(); // Channel from io to MAIN
        let (tx_i, rx_i) = channel(); // Channel from main to IO
        let input_tx = tx_i.clone();
        spawn_input_thread(input_tx);
        IoThread {
            tx: tx_i,
            rx: rx_m,
            handle: spawn(move || display_thread(tx_m, rx_i)),
        }
    }
}

impl TuiApp {
    /// Public event handler for the TuiApp. Should be run between each render.
    pub fn handle_event(&mut self) -> bool {
        if self.paused || self.finished {
            match self.rx.recv() {
                Ok(e) => return self.process_event(e),
                Err(_) => Exit::IoThreadError.exit(
                    Some("Input Thread stopped communicating properly.")
                ),
            }
        } else {
            match self.rx.try_recv() {
                Ok(e) => return self.process_event(e),
                Err(TryRecvError::Disconnected) => Exit::IoThreadError.exit(
                    Some("Input Thread went missing, assumed dead.")
                ),
                _ => return true,
            }
        }
    }

    /// Adds a simulator state to the history in the TuiApp state.
    fn add_state(&mut self, state: State) {
        self.states.push_front(state);
        if self.states.len() > KEPT_STATES {
            self.states.pop_back();
        }
    }

    /// Process an IoEvent.
    fn process_event(&mut self, event: IoEvent) -> bool {
        match event {
            IoEvent::Exit => return false,
            IoEvent::Finish => self.finished = true,
            IoEvent::Input(k) => self.process_key(k),
            IoEvent::UpdateState(s) => self.add_state(s),
        };
        true
    }

    /// Process a key input.
    fn process_key(&mut self, key: Key) {
        match key {
            Key::Char(' ') => self.toggle_pause(),
            Key::Left => self.state_backward(),
            Key::Right => self.state_forward(),
            _ => {},
        }
    }

    /// Rewinds the state to the last one in the history.
    fn state_backward(&mut self) {
        if self.hist_display == 0 && (!self.paused || self.finished) {
            self.toggle_pause();
        }
        self.hist_display = cmp::min(
            cmp::min(self.hist_display + 1, KEPT_STATES),
            self.states.len()
        );
    }

    /// Rewinds the state to the last one in the history.
    fn state_forward(&mut self) {
        if self.hist_display > 0 {
            self.hist_display -= 1;
        } else if !self.finished {
            self.tx.send(SimulatorEvent::Cycle).unwrap();
        }
    }

    /// Toggles whether or not the simulation is paused, instructing the
    /// simulator on what to do.
    fn toggle_pause(&mut self) {
        if !self.finished && self.hist_display == 0 {
            self.tx.send(SimulatorEvent::PauseToggle).unwrap();
            self.paused = !self.paused;
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Main entry point for the display thread that handles display updates and
/// user input.
fn display_thread(
    tx: Sender<SimulatorEvent>,
    rx: Receiver<IoEvent>,
) {
    // Initalise
    let mut terminal = new_terminal().expect("Could not start fancy UI.");
    let mut app = TuiApp {
        tx,
        rx,
        size: Rect::default(),
        states: VecDeque::new(),
        finished: false,
        paused: INITIALLY_PAUSED,
        hist_display: 0,
    };

    terminal.hide_cursor().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        // Draw output
        match draw_state(&mut terminal, &app) {
            Ok(()) => (),
            Err(_) => Exit::IoThreadError.exit(
                Some("Error when drawing simulation state. {:?}")
            ),
        }

        if !app.handle_event() {
            break
        }
    }

    #[allow(unused_must_use)]
    {
        app.tx.send(SimulatorEvent::Finish);
        terminal.clear();
    }

    // Unknown bug, dirty fix:
    // For unknown reasons, occasionally the terminal isn't dropped when we
    // break out of the display thread loop, meaning the terminal settings are
    // not reset. Explicit call to drop just in case.
    std::mem::drop(terminal)
}

