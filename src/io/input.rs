use std::io;
use std::sync::mpsc::Sender;
use std::thread::{JoinHandle, spawn};

use termion::event::Key;
use termion::input::TermRead;

use super::IoEvent;

///////////////////////////////////////////////////////////////////////////////
//// CONST/STATIC

/// The key presses that will exit the simulator.
const EXIT_KEYS: [Key; 4] = [
    Key::Esc,
    Key::Char('q'),
    Key::Ctrl('c'),
    Key::Ctrl('d'),
];

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Spawns the input handler thread. This thread will run in the background
/// and send key press/exit events to the given Sender.
pub fn spawn_input_thread(tx: Sender<IoEvent>) -> JoinHandle<()> {
    spawn(move || input_thread(&tx))
}

/// Function for handling user input, called within it's own thread as this
/// will loop until either it fails to send an input event, or an exit button
/// is pressed.
fn input_thread(tx: &Sender<IoEvent>) {
    let stdin = io::stdin();
    for evt in stdin.keys() {
        if let Ok(key) = evt {
            if EXIT_KEYS.contains(&key) {
                if tx.send(IoEvent::Exit).is_err() {
                    return;
                }
            } else if tx.send(IoEvent::Input(key)).is_err() {
                return;
            }
        }
    }
}

