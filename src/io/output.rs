use std::cmp;
use std::io::{stdout, Error, Stdout};

use byteorder::{LittleEndian, ReadBytesExt};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::{Frame, Terminal as TuiTerminal};

use crate::isa::Instruction;
use crate::isa::operand::Register;
use crate::simulator::state::State;

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
    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = TuiTerminal::new(backend)?;
    Ok(terminal)
}

/// Entry point for the drawing of the current stored simulate state.
pub fn draw_state(terminal: &mut Terminal, app: &TuiApp) -> std::io::Result<()> {
    terminal.draw(|mut f| {
        let horz_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(app.size);
        let vert_chunks_0 = Layout::default()
            .direction(Direction::Vertical)
            . constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]
                .as_ref()
            )
            .split(horz_chunks[0]);
        let vert_chunks_2 = Layout::default()
            .direction(Direction::Vertical)
            . constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ]
                .as_ref()
            )
            .split(horz_chunks[2]);
        let default = State::default();
        draw_stats(&mut f, vert_chunks_0[0], &app, &default);
        draw_debug(&mut f, vert_chunks_0[1], &app, &default);
        draw_registers(&mut f, horz_chunks[1], &app, &default);
        draw_instr_memory(&mut f, vert_chunks_2[0], &app, &default);
        draw_stack_memory(&mut f, vert_chunks_2[1], &app, &default);
    })
}

/// Draws the TuiApp state statistics on screen.
fn draw_stats(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let epc = if state.stats.cycles == 0 {
        0f32
    } else {
        state.stats.executed as f32 / state.stats.cycles as f32
    };
    let bpr = if state.stats.bp_success + state.stats.bp_failure == 0 {
        0f32
    } else {
        state.stats.bp_success as f32 / (state.stats.bp_success + state.stats.bp_failure) as f32
    };
    let mut tmp: Vec<Text> = vec![
        Text::raw(format!("executed: {}\n", state.stats.executed)),
        Text::raw(format!("cycles:   {}\n", state.stats.cycles)),
        Text::raw(format!("avg. executions/cycle: {:.3}\n", epc)),
        Text::raw(format!("stalls:   {}\n", state.stats.stalls)),
        Text::raw(format!("bp_succ:  {}\n", state.stats.bp_success)),
        Text::raw(format!("bp_fail:  {}\n", state.stats.bp_failure)),
        Text::raw(format!("bp_rate:  {}\n", bpr)),
        Text::raw("\n"),
        Text::raw(format!("LF: {:x?}\n", state.latch_fetch)),
        Text::raw("\n"),
        Text::raw("RS:\n"),
    ];
    for resv in state.resv_station.contents.iter() {
        tmp.push(Text::raw(format!("{:?}\n", resv)));
    }
    tmp.push(Text::raw("\n"));
    tmp.push(Text::raw("EU's\n"));
    for eu in state.execute_units.iter() {
        tmp.push(Text::raw(format!("{:?}: {:?}\n", eu.unit_type, eu.executing)));
    }
    tmp.push(Text::raw("\n"));
    tmp.push(Text::raw("RB:\n"));
    let mut i = state.reorder_buffer.front;
    while i % state.reorder_buffer.capacity != state.reorder_buffer.back {
        tmp.push(
            Text::raw(
                format!(
                    "{:02}: {:?}\n",
                    i,
                    state.reorder_buffer.rob[i % state.reorder_buffer.capacity]
                )
            )
        );
        i += 1;
    }
    Paragraph::new(tmp.iter())
        .block(standard_block("Statistics"))
        .wrap(true)
        .render(f, area);
}

/// Draws the TuiApp state statistics on screen.
fn draw_debug(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let messages: Vec<Text> = state
        .debug_msg
        .iter()
        .map(|str| Text::raw(format!("{}\n", str)))
        .collect();
    Paragraph::new(messages.iter())
        .block(standard_block("Debug Prints"))
        .wrap(true)
        .render(f, area);
}

/// Draws the register file.
fn draw_registers(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state_prev = app.states.get(app.hist_display + 1).unwrap_or(default);
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let registers = state.register.file.iter().enumerate().map(|(name, are)| {
        let reg = Register::from(name as i32);
        let val_a = are.data;
        let val_a_prev = state_prev.register.file[name].data;
        let val_p = if are.rename.is_some() {
            state.reorder_buffer[are.rename.unwrap()].act_rd.unwrap_or(0)
        } else {
            0
        };
        let val_p_prev = if are.rename.is_some() {
            state_prev.reorder_buffer[are.rename.unwrap()].act_rd.unwrap_or(0)
        } else {
            0
        };
        Text::styled(
            format!(
                "{n:>#04}-{n:<03} ({rn}) :: {va:08x}|{vp:08x} - {va}/{vp}",
                n=reg,
                va=val_a,
                vp=val_p,
                rn=if are.rename.is_none() {
                    String::from("  ")
                } else {
                    format!("{:02}", are.rename.unwrap())
                },
            ),
            if reg == Register::PC {
                Style::default().fg(Color::LightBlue).modifier(Modifier::Bold)
            } else if val_a != val_a_prev || val_p != val_p_prev {
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

/// Draws a section of the memory around the PC.
fn draw_instr_memory(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let pc = state.branch_predictor.lc as i32;
    let skip_amount = cmp::max(0, (pc - (i32::from(4 * area.height) / 2)) / 4) as usize;
    let memory = state
        .memory
        .chunks(4)
        .enumerate()
        .skip(skip_amount)
        .map(|(mut addr, mut value)| {
            addr *= 4;
            let word = value.read_i32::<LittleEndian>().unwrap();
            Text::styled(
                match Instruction::decode(word) {
                    Some(i) => format!("{a:08x} :: {v:08x} - {i}", a = addr, v = word, i = i,),
                    None => format!("{a:08x} :: {v:08x} - {v}", a = addr, v = word,),
                },
                if addr as i32 == pc {
                    Style::default()
                        .fg(Color::LightBlue)
                        .modifier(Modifier::Bold)
                } else {
                    Style::default().fg(Color::White)
                },
            )
        });

    List::new(memory)
        .block(standard_block("Memory (Centred LC)"))
        .render(f, area);
}

fn draw_stack_memory(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let sp = state.register[Register::X2].data;
    let skip_amount = cmp::max(0, (sp - (i32::from(4 * area.height) / 2)) / 4) as usize;
    let memory = state
        .memory
        .chunks(4)
        .enumerate()
        .skip(skip_amount)
        .map(|(mut addr, mut value)| {
            addr *= 4;
            let word = value.read_i32::<LittleEndian>().unwrap();
            Text::styled(
                format!("{a:08x} :: {v:08x} - {v}", a = addr, v = word),
                if addr as i32 == sp {
                    Style::default()
                        .fg(Color::LightBlue)
                        .modifier(Modifier::Bold)
                } else {
                    Style::default().fg(Color::White)
                },
            )
        });

    List::new(memory)
        .block(standard_block("Memory (Centred SP)"))
        .render(f, area);
}

/// Constructs a standardised Block widget with given title.
pub fn standard_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title_style(
            Style::default()
                .fg(Color::LightGreen)
                .modifier(Modifier::Bold),
        )
        .title(title)
}
