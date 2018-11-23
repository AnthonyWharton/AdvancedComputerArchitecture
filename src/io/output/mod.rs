use std::collections::VecDeque;
use std::io::{Error, Stdout, stdout};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use tui::Terminal as TuiTerminal;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use termion::raw::{IntoRawMode, RawTerminal};

use simulator::state::State;
use util::exit::Exit;
use super::{IoEvent, SimulatorEvent};
use super::input::{InputHandler, EXIT_KEYS};

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// For dealing with state output updates.
mod state;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

/// The number of states to keep in memory.
/// Each state uses approximately O(sim_mem_size) RAM, which is typically 1mb.
const KEPT_STATES: usize = 100;

///////////////////////////////////////////////////////////////////////////////
//// TYPES

type Terminal = TuiTerminal<TermionBackend<RawTerminal<Stdout>>>;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Encapsulation of the state for the TuiApp front-end.
pub struct TuiApp {
    input_handler: InputHandler,
    size: Rect,
    states: VecDeque<State>,
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Connstructs a new raw terminal for TUI/Terminon usage.
fn new_terminal() -> Result<Terminal, Error> {
    let stdout   = stdout().into_raw_mode()?;
    let backend  = TermionBackend::new(stdout);
    let terminal = TuiTerminal::new(backend)?;
    Ok(terminal)
}

/// Main entry point for the display thread that handles display updates and
/// user input.
pub fn display_thread(
    tx: Sender<SimulatorEvent>,
    rx: Receiver<IoEvent>
) {
    // Initalise
    let mut terminal = new_terminal().expect("Could not start fancy UI.");
    let mut app = TuiApp {
        input_handler: InputHandler::new(),
        size: Rect::default(),
        states: VecDeque::new(),
    };

    terminal.hide_cursor().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        // Deal with input
        match app.input_handler.next() {
            Ok(key) => match key {
                k if EXIT_KEYS.contains(&k) => break,
                _ => {},
            }
            Err(TryRecvError::Disconnected) => Exit::IoThreadError.exit(
                Some("Input Thread went missing, assumed dead.")
            ),
            _ => {},
        }

        // Deal with recieved events
        match rx.try_recv() {
            Ok(e) => match e {
                IoEvent::Finish => break,
                IoEvent::DoneThing => println!("Done thing.\r"),
                IoEvent::UpdateState(s) => {
                    add_state(&mut app, s);
                    state::simple_draw_state(app.states.front().unwrap())
                },
            },
            Err(TryRecvError::Disconnected) => 
                Exit::IoThreadError.exit(Some("Simulator thread missing, assumed dead.")),
            _ => {},
        }
    }

    match tx.send(SimulatorEvent::Finish) {
        _ => {},
    }
}

fn add_state(app: &mut TuiApp, state: State) {
    app.states.push_front(state);
    if app.states.len() > KEPT_STATES {
        app.states.pop_back();
    }
}

