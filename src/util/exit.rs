use std::process;

pub enum Exit {
    ArgumentError,
    FileLoadError,
    ElfError,
    IoThreadError,
}

impl Exit {
    pub fn exit(self, message: Option<&str>) -> ! {
        match self {
            Exit::ArgumentError => exit(1, message.unwrap_or("Argument Error!")),
            Exit::FileLoadError => exit(2, message.unwrap_or("File Load Error!")),
            Exit::ElfError      => exit(3, message.unwrap_or("Elf File Error!")),
            Exit::IoThreadError => exit(4, message.unwrap_or("IO Thread Error!")),
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

