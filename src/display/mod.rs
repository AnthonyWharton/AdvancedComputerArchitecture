use std::io::{Error, Stdout, stdout};
use tui::Terminal as TuiTerminal;
use tui::backend::TermionBackend;
use termion::raw::{IntoRawMode, RawTerminal};

///////////////////////////////////////////////////////////////////////////////
//// EXTERNAL MODULES

/// User Input event handler logic.
pub mod input;

///////////////////////////////////////////////////////////////////////////////
//// TYPES

type Terminal = TuiTerminal<TermionBackend<RawTerminal<Stdout>>>;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

pub fn initialize() -> Result<Terminal, Error> {
    let stdout  = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = TuiTerminal::new(backend)?;
    Ok(terminal)
}
