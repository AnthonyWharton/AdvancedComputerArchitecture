use std::process;

pub enum Exit {
    ArgumentError,
}

impl Exit {
    pub fn exit(self, message: Option<&'static str>) -> ! {
        match self {
            Exit::ArgumentError => exit(1, message.unwrap_or("Argument Error!")),
        }
    }
}

fn exit(code: i32, message: &'static str) -> ! {
    if message.len() != 0 {
        if code != 0 {
            println!("Error: {}", message);
        } else {
            println!("{}", message);
        }
    }
    process::exit(code);
}
