use std::cmp;
use std::io::{stdout, Error, Stdout};

use byteorder::{LittleEndian, ReadBytesExt};
use either::{Left, Right};
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
        let default = State::default();
        let horz_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20), // Stats, Fetch Latch, Reg File
                    Constraint::Percentage(58), // ResvStation, ROB
                    Constraint::Percentage(22), // Memory Column
                ]
                .as_ref(),
            )
            .split(app.size);

        /////////////////////////////////////////////////////////// LEFT COLUMN
        let left_col = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(9),
                    Constraint::Length(6),
                    Constraint::Min(33),
                ]
                .as_ref()
            )
            .split(horz_chunks[0]);
        draw_stats(&mut f, left_col[0], &app, &default);
        draw_latch_fetch(&mut f, left_col[1], &app, &default);
        draw_registers(&mut f, left_col[2], &app, &default);

        ///////////////////////////////////////////////////////// CENTRE COLUMN
        let centre_col = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]
                .as_ref()
            )
            .split(horz_chunks[1]);
        let rsv_rob_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(45),
                    Constraint::Percentage(55),
                ]
                .as_ref()
            )
            .split(centre_col[0]);
        draw_reservation_station(&mut f, rsv_rob_split[0], &app, &default);
        draw_reorder_buffer(&mut f, rsv_rob_split[1], &app, &default);
        draw_debug(&mut f, centre_col[1], &app, &default);

        ////////////////////////////////////////////////////////// RIGHT COLUMN
        let right_col = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ]
                .as_ref()
            )
            .split(horz_chunks[2]);
        draw_instr_memory(&mut f, right_col[0], &app, &default);
        draw_stack_memory(&mut f, right_col[1], &app, &default);
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
    ];
    tmp.push(Text::raw("EU's\n"));
    for eu in state.execute_units.iter() {
        tmp.push(Text::raw(format!("{:?}: {:?}\n", eu.unit_type, eu.executing)));
    }
    tmp.push(Text::raw("\n"));
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
        let val = are.data;
        let val_prev = state_prev.register.file[name].data;
        Text::styled(
            format!(
                "{n:>#04}-{n:<03} ({rn}) :: {v:08x} - {v}",
                n=reg,
                v=val,
                rn=if are.rename.is_none() {
                    String::from("  ")
                } else {
                    format!("{:02}", are.rename.unwrap())
                },
            ),
            if reg == Register::PC {
                Style::default().fg(Color::LightBlue).modifier(Modifier::Bold)
            } else if val != val_prev {
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

/// Draws the fetch latch
fn draw_latch_fetch(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let lf = &state.latch_fetch;
    let messages: Vec<Text> = vec![
        Text::raw(format!("{:08x}: {}", lf.pc, format_option!("{}", lf.data))),
    ];
    Paragraph::new(messages.iter())
        .block(standard_block("Fetch Latch"))
        .wrap(true)
        .render(f, area);
}

/// Draws the reservation station.
fn draw_reservation_station(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let rsv = &state.resv_station;
    let rob = &state.reorder_buffer;
    let list = rsv.contents.iter().enumerate().map(|(n, e)| {
        let ready = match e.rs1 {
            Left(_)  => true,
            Right(n) => rob[n].act_rd.is_some(),
        }
        &&
        match e.rs2 {
            Left(_)  => true,
            Right(n) => rob[n].act_rd.is_some(),
        };
        Text::styled(
            format!("{:02}: {}", n, e),
            if ready {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            }
        )
    });

    List::new(list)
        .block(standard_block("Unified Reservation Station"))
        .render(f, area);
}

/// Draws the reorder buffer.
fn draw_reorder_buffer(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let rob = &state.reorder_buffer;
    let len = rob.rob.len();
    let list = rob.rob.iter().enumerate().map(|(n, e)| {
        let nc = (n + len) % len;
        Text::styled(
            format!("{:02}: {}", n, e),
            if (rob.front_fin + len) % len <= nc && nc < (rob.back + len) % len {
                Style::default().fg(Color::White)
            } else if (rob.front + len) % len <= nc && nc < (rob.back + len) % len {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            }
        )
    });

    List::new(list)
        .block(standard_block("Reorder Buffer"))
        .render(f, area);
}

/// Draws a section of the memory around the Load Counter.
fn draw_instr_memory(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let pc = state.branch_predictor.lc as i32;
    let skip_amount = cmp::max(0, (pc - (i32::from(4 * area.height) / 2)) / 4) as usize + 1;
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
        .block(standard_block("Memory (Centred on Load Counter)"))
        .render(f, area);
}

/// Draws a section of the memory around the Load Counter.
fn draw_stack_memory(f: &mut Frame<Backend>, area: Rect, app: &TuiApp, default: &State) {
    let state = app.states.get(app.hist_display).unwrap_or(default);
    let sp_c = state.register[Register::X2].data;
    let last = app
        .states
        .iter()
        .skip(1)
        .find(|s| s.register[Register::X2].data > sp_c)
        .unwrap_or(state);
    let sp_a = if last.register[Register::X2].data == sp_c {
        i32::max_value()
    } else {
        last.register[Register::X2].data
    };
    let skip_amount = sp_c as usize / 4;
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
                if sp_c <= (addr as i32) && (addr as i32) < sp_a {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::DarkGray)
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
