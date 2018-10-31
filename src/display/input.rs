use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

pub const EXIT_KEYS: [Key; 2] = [Key::Esc, Key::Char('q')];

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

pub struct InputHandler {
    rx:           mpsc::Receiver<Key>,
    input_handle: thread::JoinHandle<()>,
}

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl InputHandler {
    pub fn new() -> InputHandler {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || handle_input(tx))
        };
        InputHandler {
            rx,
            input_handle,
        }
    }

    pub fn next(&self) -> Result<Key, mpsc::RecvTimeoutError> {
        self.rx.recv_timeout(Duration::from_millis(10))
    }
}

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Function for handling user input, called within it's own thread as this 
/// will loop until either it fails to send an input event, or an exit button
/// is pressed.
fn handle_input(tx: mpsc::Sender<Key>) {
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

