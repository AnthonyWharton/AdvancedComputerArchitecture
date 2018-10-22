use std::process;

pub enum Exit {
    ArgumentError,
    FileLoadError,
}

impl Exit {
    pub fn exit(self, message: Option<&str>) -> ! {
        match self {
            Exit::ArgumentError => exit(1, message.unwrap_or("Argument Error!")),
            Exit::FileLoadError => exit(1, message.unwrap_or("File Load Error!")),
        }
    }
}

fn exit(code: i32, message: &str) -> ! {
    if message.len() != 0 {
        if code != 0 {
            println!("Error: {}", message);
        } else {
            println!("{}", message);
        }
    }
    process::exit(code);
}
