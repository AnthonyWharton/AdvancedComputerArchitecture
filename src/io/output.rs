use std::cmp;
use std::io::{Error, Stdout, stdout};

use byteorder::{LittleEndian, ReadBytesExt};
use super::termion::raw::{IntoRawMode, RawTerminal};
use super::tui::{Frame, Terminal as TuiTerminal};
use super::tui::backend::TermionBackend;
use super::tui::layout::{Constraint, Direction, Layout, Rect};
use super::tui::style::{Color, Modifier, Style};
use super::tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};

use isa::Instruction;
// use isa::operand::Register;
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
        draw_stats(&mut f, chunks[0], &app, &State::default());
        draw_registers(&mut f, chunks[1], &app, &State::default());
        draw_memory(&mut f, chunks[2], &app, &State::default());
    })
}

/// Draws the TuiApp state statistics on screen.
fn draw_stats(
    f: &mut Frame<Backend>,
    area: Rect,
    app: &TuiApp,
    default: &State
) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let epc = if state.stats.cycles == 0 {
        0f32
    } else {
        state.stats.executed as f32/ state.stats.cycles as f32
    };
    let tmp = vec![
        Text::raw(format!("executed: {}\n", state.stats.executed)),
        Text::raw(format!("cycles:   {}\n", state.stats.cycles)),
        Text::raw(format!("avg. executions/cycle: {:.3}\n", epc)),
        Text::raw("\n"),
    ];
    Paragraph::new(tmp.iter())
        .block(standard_block("Statistics"))
        .wrap(true)
        .render(f, area);
}

/// Draws the register file.
fn draw_registers(
    _f: &mut Frame<Backend>,
    _area: Rect,
    app: &TuiApp,
    default: &State
) {
    let _state_prev = app.states.get(app.hist_display+1).unwrap_or(default);
    let _state = app.states.get(app.hist_display).unwrap_or(default);
    // let registers = state.register.iter().enumerate().map(|(name, value)| {
    //     let reg = Register::from(name as i32);
    //     Text::styled(
    //         format!(
    //             "{n:>#04}-{n:<03} :: {v:08x} - {v}",
    //             n=reg,
    //             v=value
    //         ),
    //         if reg == Register::PC {
    //             Style::default().fg(Color::LightBlue).modifier(Modifier::Bold)
    //         } else if state.register.read_at_name(name).unwrap() != state_prev.register.read_at_name(name).unwrap() {
    //             Style::default().fg(Color::Black).bg(Color::LightYellow)
    //         } else {
    //             Style::default().fg(Color::White)
    //         }
    //     )
    // });

    // List::new(registers)
    //     .block(standard_block("Register File"))
    //     .render(f, area);
}

/// Draws a section of the memory around the PC.
fn draw_memory(
    f: &mut Frame<Backend>,
    area: Rect,
    app: &TuiApp,
    default: &State
) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let pc = state.branch_predictor.get_prediction() as i32;
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

/// Constructs a standardised Block widget with given title.
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

