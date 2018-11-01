extern crate backtrace;

use std::env;
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
        attempt_cleanup_raw_terminal();
    }));
}

fn attempt_cleanup_raw_terminal() {
    let mut status = 0;
    match process::Command::new("setterm").arg("-reset").output() {
        Ok(o) => status += if o.status.success() { 0 } else { 1 },
        _     => status += 1,
    };
    match process::Command::new("reset").output() {
        Ok(o) => status += if o.status.success() { 0 } else { 1 },
        _     => status += 1,
    }
    match process::Command::new("stty").arg("sane").output() {
        Ok(o) => status += if o.status.success() { 0 } else { 1 },
        _     => status += 1,
    }
    if status != 0 {
        print!("!!!!!!!!! <WARNING> !!!!!!!!!\n\r");
        print!("May have failed to clean raw terminal configuration. If ");
        print!("your terminal is broken/has no cursor, try all/some of:\n\r");
        print!("  - `reset`\n\r");
        print!("  - `stty sane`\n\r");
        print!("  - `setterm --reset\n\r");
        print!("  - `echo -e \"\\033c\"\n\r` ");
        print!("...to /hopefully/ fix your terminal.\n\r");
        print!("If those don't work, good luck! (I recommend restarting ");
        print!("your terminal)... And sorry!\n\r");
        print!("!!!!!!!!! </WARNING> !!!!!!!!\n\r");
    }
}

/// Essentially a clone of the backtrace library `Debug::fmt` implementation,
/// but with `\r`'s to deal with the raw terminal.
fn print_backtrace() {
    match env::var_os("RUST_BACKTRACE") {
        Some(ref s) if s == "1" => {},
        _ => {
            println!("Set RUST_BACKTRACE=1 to print a backtrace.\r");
            return;
        },
    }
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
