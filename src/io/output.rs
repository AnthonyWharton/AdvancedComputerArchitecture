use std::cmp;
use std::collections::VecDeque;
use std::io::{Error, Stdout, stdout};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use byteorder::{LittleEndian, ReadBytesExt};
use tui::{Frame, Terminal as TuiTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};

use isa::Instruction;
use isa::operand::Register;
use simulator::state::State;
use util::exit::Exit;
use super::{IoEvent, SimulatorEvent};
use super::input::{InputHandler, EXIT_KEYS};
// use super::state;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

/// The number of states to keep in memory.
/// Each state uses approximately O(sim_mem_size) RAM, which is typically 1mb.
const KEPT_STATES: usize = 100;

///////////////////////////////////////////////////////////////////////////////
//// TYPES

/// Type alias for abbreviating the long Termion Backend type
type Backend = TermionBackend<RawTerminal<Stdout>>;

/// Type alias for abbreviating the Terminal type
type Terminal = TuiTerminal<Backend>;

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

/// Encapsulation of the state for the TuiApp front-end.
pub struct TuiApp {
    /// Thread for input handling
    input_handler: InputHandler,
    /// Terminal size
    size: Rect,
    /// History of the last KEPT_STATES states
    states: VecDeque<State>,
    /// Whether or not the simulator has finished
    finished: bool,
    /// Whether or not the simulator is paused
    paused: bool,
    /// Which historical state we are showing.
    /// 0 is current, 1 is the state before, 2 is the state before 1, etc
    hist_display: u8,
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
pub fn display_thread(tx: Sender<SimulatorEvent>, rx: Receiver<IoEvent>) {
    // Initalise
    let mut terminal = new_terminal().expect("Could not start fancy UI.");
    let mut app = TuiApp {
        input_handler: InputHandler::new(),
        size: Rect::default(),
        states: VecDeque::new(),
        finished: false,
        paused: false,
        hist_display: 0,
    };

    terminal.hide_cursor().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        // Deal with input, non-blocking if simulation is running, else block
        // when listening to yeild host processor time
        if !app.finished || app.paused {
            match app.input_handler.try_next() {
                Ok(key) => if process_key(key, &mut app, &tx) { break },
                Err(TryRecvError::Disconnected) => Exit::IoThreadError.exit(
                    Some("Input Thread went missing, assumed dead.")
                ),
                _ => {},
            }
        } else { 
            match app.input_handler.next() {
                Ok(key) => if process_key(key, &mut app, &tx) { break },
                Err(_) => Exit::IoThreadError.exit(
                    Some("Input Thread stopped communicating properly.")
                ),
            }
        }

        // Deal with recieved events
        match rx.try_recv() {
            Ok(e) => match e {
                IoEvent::Finish => app.finished = true,
                IoEvent::UpdateState(s) => {
                    add_state(&mut app, s);
                    // state::simple_draw_state(app.states.front().unwrap())
                    match draw_state(&mut terminal, &app) {
                        Ok(()) => (),
                        Err(_) => Exit::IoThreadError.exit(
                            Some("Error when drawing simulation state. {:?}")
                        ),
                    }
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

    #[allow(unused_must_use)]
    { terminal.clear(); }

    // Unknown bug, dirty fix:
    // For unknown reasons, occasionally the terminal isn't dropped when we 
    // break out of the display thread loop, meaning the terminal settings are 
    // not reset. Explicit call to drop just in case.
    std::mem::drop(terminal)
}

/// Process a key input
fn process_key(
    key: Key,
    app: &mut TuiApp,
    tx: &Sender<SimulatorEvent>
) -> bool {
    match key {
        k if EXIT_KEYS.contains(&k) => return true,
        Key::Char(' ') => {
            if !app.finished {
                tx.send(SimulatorEvent::PauseToggle).unwrap();
                app.paused = !app.paused;
            }
        }
        _ => {},
    }
    false
}

/// Adds a simulator state to the history in the TuiApp state.
fn add_state(app: &mut TuiApp, state: State) {
    app.states.push_front(state);
    if app.states.len() > KEPT_STATES {
        app.states.pop_back();
    }
}

/// Entry point for the drawing of the current stored simulate state.
fn draw_state(terminal: &mut Terminal, app: &TuiApp) -> std::io::Result<()> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ].as_ref()
            )
            .split(app.size);
        Block::default()
            .title("Foo")
            .borders(Borders::ALL)
            .render(&mut f, chunks[0]);
        draw_registers(&mut f, chunks[1], &app, &State::default());
        draw_memory(&mut f, chunks[2], &app, &State::default());
    })
}

/// Draws the register block
fn draw_registers(
    f: &mut Frame<Backend>,
    area: Rect,
    app: &TuiApp,
    default: &State
) {
    let map_closure = |(name, value)| { 
        Text::styled(
            format!(
                "{n:>#04}-{n:<03} = {v:08x} ({v:11})", 
                n=Register::from(name as i32), 
                v=value
            ),
            Style::default().fg(Color::White)
        )
    };
 
    let state = app.states.front().unwrap_or(default);
    let registers = state.register.iter().enumerate().map(map_closure);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title_style(Style::default().fg(Color::LightGreen))
        .title("Register File");

    List::new(registers)
        .block(block)
        .render(f, area);
}

/// Draws the memory block
fn draw_memory(
    f: &mut Frame<Backend>,
    area: Rect,
    app: &TuiApp,
    default: &State
) {
    let state = app.states.front().unwrap_or(default);
    let pc = state.register[Register::PC as usize];
    let mut skip_amount = cmp::max(0, pc - ((area.height as i32) / 2)) as usize;
    skip_amount /= 4;
    skip_amount -= skip_amount % area.height as usize;
    let memory = state.memory
        .chunks(4)
        .enumerate()
        .map(|(mut addr, mut value)| { 
            addr = addr * 4;
            let word = value.read_i32::<LittleEndian>().unwrap();
            Text::styled(
                match Instruction::decode(word) {
                    Some(i) => {
                        format!(
                            "{a:08x} = {v:08x} - {i}", 
                            a=addr, 
                            v=word,
                            i=i,
                        )
                    },
                    None => {
                        format!(
                            "{a:08x} = {v:08x} - {v}", 
                            a=addr, 
                            v=word,
                        )
                    }
                },
                if addr as i32 == pc {
                    Style::default().fg(Color::LightBlue) 
                } else {
                    Style::default().fg(Color::White)
                }
            )
        }).skip(skip_amount);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title_style(Style::default().fg(Color::LightGreen))
        .title("Memory");

    List::new(memory)
        .block(block)
        .render(f, area);
}
