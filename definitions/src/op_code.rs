pub enum OpCode {
    // Arithmentic Operations
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,

    // Logical Operations
    NOT,
    AND,
    ORR,
    XOR,

    // Loading
    LDR,
    SET,
    RND,
    PSH,
    POP,

    // Branching
    BRU,
    BRZ,
    BRN,
    BRP,
    FUN,
    RET,

    // Misc
    HCF
}

impl OpCode {
    pub fn as_str(&self) -> &str {
        match self {
            OpCode::ADD => "ADD",
            OpCode::SUB => "SUB",
            OpCode::MUL => "MUL",
            OpCode::DIV => "DIV",
            OpCode::MOD => "MOD",
            
            OpCode::NOT => "NOT",
            OpCode::AND => "AND",
            OpCode::ORR => "ORR",
            OpCode::XOR => "XOR",

            OpCode::LDR => "LDR",
            OpCode::SET => "SET",
            OpCode::RND => "RND",
            OpCode::PSH => "PSH",
            OpCode::POP => "POP",

            OpCode::BRU => "BRU",
            OpCode::BRZ => "BNZ",
            OpCode::BRN => "BRN",
            OpCode::BRP => "BRP",
            OpCode::FUN => "FUN",
            OpCode::RET => "RET",

            OpCode::HCF => "HCF"
        }
    }

    pub fn as_i8(&self) -> i8 {
        match self {
            OpCode::ADD => 0x00,
            OpCode::SUB => 0x01,
            OpCode::MUL => 0x02,
            OpCode::DIV => 0x03,
            OpCode::MOD => 0x04,
            
            OpCode::NOT => 0x05,
            OpCode::AND => 0x06,
            OpCode::ORR => 0x07,
            OpCode::XOR => 0x08,

            OpCode::LDR => 0x09,
            OpCode::SET => 0x0a,
            OpCode::RND => 0x0b,
            OpCode::PSH => 0x0c,
            OpCode::POP => 0x0e,

            OpCode::BRU => 0x0e,
            OpCode::BRZ => 0x0f,
            OpCode::BRN => 0x10,
            OpCode::BRP => 0x11,
            OpCode::FUN => 0x12,
            OpCode::RET => 0x13,

            OpCode::HCF => 0x7f
        }
    }
}

