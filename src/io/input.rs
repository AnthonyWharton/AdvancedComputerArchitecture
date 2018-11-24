use std::io;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

use util::exit::Exit;
use super::{EXIT_KEYS, TuiApp};

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

pub struct InputHandler(Receiver<Key>);

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl InputHandler {
    pub fn new() -> InputHandler {
        let (tx, rx) = channel();
        let tx = tx.clone();
        thread::spawn(move || input_thread(tx));
        InputHandler(rx)
    }

    /// Deal with input, non-blocking if simulation is running, blocking when
    /// simulation isn't running so as to yeild host processor time.
    /// Returns whether or not the user pressed an exit key.
    pub fn next(&self, app: &mut TuiApp) -> bool {
        if app.paused || app.finished {
            match self.0.recv() {
                Ok(key) => return app.process_key(key),
                Err(_) => Exit::IoThreadError.exit(
                    Some("Input Thread stopped communicating properly.")
                ),
            }
        } else {
            match self.0.try_recv() {
                Ok(key) => return app.process_key(key),
                Err(TryRecvError::Disconnected) => Exit::IoThreadError.exit(
                    Some("Input Thread went missing, assumed dead.")
                ),
                _ => {},
            }
        }
        false
    }
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Function for handling user input, called within it's own thread as this
/// will loop until either it fails to send an input event, or an exit button
/// is pressed.
fn input_thread(tx: Sender<Key>) {
    let stdin = io::stdin();
    for evt in stdin.keys() {
        match evt {
            Ok(key) => {
                if let Err(_) = tx.send(key) {
                    return;
                }
                if EXIT_KEYS.contains(&key) {
                    return;
                }
            }
            Err(_) => {}
        }
    }
}

