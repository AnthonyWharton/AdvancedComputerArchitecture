extern crate backtrace;

use std::mem;
use std::panic;
use std::process;

use self::backtrace::Backtrace;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Add the panic hook that is used to make a panic message that doesn't suck
/// with the raw terminal. Will also attempt to cleanup the terminal with 
/// `reset`.
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|i| {
        println!("Panicked at: {}:{}:{}\r",
                 i.location().unwrap().file(),
                 i.location().unwrap().line(),
                 i.location().unwrap().column());
        print_backtrace();
        if process::Command::new("reset").output().is_err() {
            print!("Failed to clean raw terminal, try `stty sane` or ");
            print!("`echo -e \"\033c\"` to fix your terminal. ");
            print!("If those don't work, good luck!\r");
        }
    }));
}

/// Essentially a clone of the backtrace library `Debug::fmt` implementation,
/// but with `\r`'s to deal with the raw terminal.
fn print_backtrace() {
    let bt = Backtrace::new();
    let hex_width = mem::size_of::<usize>() * 2 + 2;
    print!("stack backtrace:");
    for (idx, frame) in bt.frames().iter().enumerate() {
        let ip = frame.ip();
        print!("\n\r{:4}: {:2$?}", idx, ip, hex_width);

        let symbols = frame.symbols();
        if symbols.len() == 0 {
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
