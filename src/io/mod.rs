use std::cmp;
use std::thread;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

use termion::event::Key;
use tui::layout::Rect;

use simulator::INITIALLY_PAUSED;
use simulator::state::State;
use util::exit::Exit;
use self::input::InputHandler;
use self::output::{draw_state, new_terminal};

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// User Input event handler logic.
pub mod input;

/// Output display logic.
pub mod output;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

/// The key presses that will exit the simulator.
const EXIT_KEYS: [Key; 4] = [
    Key::Esc,
    Key::Char('q'),
    Key::Ctrl('c'),
    Key::Ctrl('d'),
];

/// The number of states to keep in memory.
/// Each state uses approximately O(sim_mem_size) RAM, which is typically 1mb.
pub const KEPT_STATES: usize = 100;

///////////////////////////////////////////////////////////////////////////////
//// ENUMS

/// Events destined for the IO thread.
#[allow(dead_code)]
pub enum IoEvent {
    /// Signal that the simulation has finished.
    Finish,
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
    pub handle: thread::JoinHandle<()>
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
    pub fn new() -> IoThread {
        let (tx_m, rx_m) = channel(); // Channel from io to MAIN
        let (tx_i, rx_i) = channel(); // Channel from main to IO
        IoThread {
            tx: tx_i,
            rx: rx_m,
            handle: thread::spawn(|| display_thread(tx_m, rx_i)),
        }
    }
}

impl TuiApp {
    /// Adds a simulator state to the history in the TuiApp state.
    fn add_state(&mut self, state: State) {
        self.states.push_front(state);
        if self.states.len() > KEPT_STATES {
            self.states.pop_back();
        }
    }

    /// Process a key input
    pub fn process_key(&mut self, key: Key) -> bool {
        match key {
            k if EXIT_KEYS.contains(&k) => return true,
            Key::Char(' ') => self.toggle_pause(),
            Key::Left => self.state_backward(),
            Key::Right => self.state_forward(),
            _ => {},
        }
        false
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
pub fn display_thread(tx: Sender<SimulatorEvent>, rx: Receiver<IoEvent>) {
    // Initalise
    let mut terminal = new_terminal().expect("Could not start fancy UI.");
    let input_handler = InputHandler::new();
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

        // Handle user input
        if input_handler.next(&mut app) {
            break;
        }

        // Deal with recieved events
        match app.rx.try_recv() {
            Ok(e) => match e {
                IoEvent::Finish => app.finished = true,
                IoEvent::UpdateState(s) => app.add_state(s),
            },
            Err(TryRecvError::Disconnected) =>
                Exit::IoThreadError.exit(Some("Simulator thread missing, assumed dead.")),
            _ => {},
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

