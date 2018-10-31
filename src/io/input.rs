use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

pub const EXIT_KEYS: [Key; 2] = [Key::Esc, Key::Char('q')];

///////////////////////////////////////////////////////////////////////////////
//// STRUCTS

pub struct InputHandler(mpsc::Receiver<Key>);

///////////////////////////////////////////////////////////////////////////////
//// IMPLEMENTATIONS

impl InputHandler {
    pub fn new() -> InputHandler {
        let (tx, rx) = mpsc::channel();
        let tx = tx.clone();
        thread::spawn(move || handle_input(tx));
        InputHandler(rx)
    }

    pub fn next(&self) -> Result<Key, mpsc::TryRecvError> {
        self.0.try_recv()
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

