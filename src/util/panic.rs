use std::env;
use std::mem;
use std::panic;
use std::process;

use backtrace::Backtrace;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Add the panic hook that is used to make a panic message that doesn't suck
/// with the raw terminal. Will also attempt to cleanup the terminal with
/// `reset`.
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|i| {
        let message = if let Some(s) = i.payload().downcast_ref::<String>() {
            s
        } else if let Some(s) = i.payload().downcast_ref::<&str>() {
            s
        } else {
            "Unable to decode panic message. You're on your own, good luck."
        };
        println!(
            "\r\n\r\nPanic! {}\r\nPanicked at: {}:{}:{}\r",
            message,
            i.location().unwrap().file(),
            i.location().unwrap().line(),
            i.location().unwrap().column()
        );
        print_backtrace();
        attempt_cleanup_raw_terminal();
    }));
}

/// Makes an effort to hard reset the terminal to get out of raw mode.
/// There seems to be no good way to reset this portably.
#[allow(unused_must_use)]
fn attempt_cleanup_raw_terminal() {
    process::Command::new("reset")
        .output()
        .expect("Failed attempt to reset terminal from raw mode.");
}

/// Essentially a clone of the backtrace library `Debug::fmt` implementation,
/// but with `\r`'s to deal with the raw terminal.
#[rustfmt::skip]
fn print_backtrace() {
    match env::var_os("RUST_BACKTRACE") {
        Some(ref s) if s == "1" => {}
        _ => {
            println!("Set RUST_BACKTRACE=1 to print a backtrace.\r");
            return;
        }
    }
    let bt = Backtrace::new();
    let hex_width = mem::size_of::<usize>() * 2 + 2;
    print!("stack backtrace:");
    for (idx, frame) in bt.frames().iter().enumerate() {
        let ip = frame.ip();
        print!("\n\r{:4}: {:2$?}", idx, ip, hex_width);

        let symbols = frame.symbols();
        if symbols.is_empty() {
            print!(" - <no info>");
        }

        for (idx, symbol) in symbols.iter().enumerate() {
            if idx != 0 {
                print!("\n\r      {:1$}", "", hex_width);
            }

            if let Some(name) = symbol.name() {
                print!(" - {}", name);
            } else {
                print!(" - <unknown>");
            }

            if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                print!("\n\r      {:3$}at {}:{}", "", file.display(), line, hex_width);
            }
        }
    }
    println!("\r");
}
