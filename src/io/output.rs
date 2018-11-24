use std::cmp;
use std::io::{Error, Stdout, stdout};

use byteorder::{LittleEndian, ReadBytesExt};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::{Frame, Terminal as TuiTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};

use isa::Instruction;
use isa::operand::Register;
use simulator::state::State;
use super::TuiApp;

///////////////////////////////////////////////////////////////////////////////
//// TYPES

/// Type alias for abbreviating the long Termion Backend type
pub type Backend = TermionBackend<RawTerminal<Stdout>>;

/// Type alias for abbreviating the Terminal type
pub type Terminal = TuiTerminal<Backend>;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Connstructs a new raw terminal for TUI/Terminon usage.
pub fn new_terminal() -> Result<Terminal, Error> {
    let stdout   = stdout().into_raw_mode()?;
    let backend  = TermionBackend::new(stdout);
    let terminal = TuiTerminal::new(backend)?;
    Ok(terminal)
}

pub fn standard_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title_style(
            Style::default()
                .fg(Color::LightGreen)
                .modifier(Modifier::Bold)
        )
        .title(title)
}

/// Entry point for the drawing of the current stored simulate state.
pub fn draw_state(terminal: &mut Terminal, app: &TuiApp) -> std::io::Result<()> {
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
        standard_block("Foo").render(&mut f, chunks[0]);
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
    let state_prev = app.states.get(app.hist_display+1).unwrap_or(default);
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let registers = state.register.iter().enumerate().map(|(name, value)| {
        let reg = Register::from(name as i32);
        Text::styled(
            format!(
                "{n:>#04}-{n:<03} :: {v:08x} - {v}",
                n=reg,
                v=value
            ),
            if reg == Register::PC {
                Style::default().fg(Color::LightBlue).modifier(Modifier::Bold)
            } else if state.register[name] != state_prev.register[name] {
                Style::default().fg(Color::Black).bg(Color::LightYellow)
            } else {
                Style::default().fg(Color::White)
            }
        )
    });

    List::new(registers)
        .block(standard_block("Register File"))
        .render(f, area);
}

/// Draws the memory block
fn draw_memory(
    f: &mut Frame<Backend>,
    area: Rect,
    app: &TuiApp,
    default: &State
) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let pc = state.register[Register::PC as usize];
    let skip_amount = cmp::max(
        0,
        (pc - ((4 * area.height as i32) / 2)) / 4
    ) as usize;
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
                            "{a:08x} :: {v:08x} - {i}",
                            a=addr,
                            v=word,
                            i=i,
                        )
                    },
                    None => {
                        format!(
                            "{a:08x} :: {v:08x} - {v}",
                            a=addr,
                            v=word,
                        )
                    }
                },
                if addr as i32 == pc {
                    Style::default().fg(Color::LightBlue).modifier(Modifier::Bold)
                } else {
                    Style::default().fg(Color::White)
                }
            )
        }).skip(skip_amount);

    List::new(memory)
        .block(standard_block("Memory"))
        .render(f, area);
}
