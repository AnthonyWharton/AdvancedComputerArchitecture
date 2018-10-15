#[derive(PartialEq)]
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
    LSH,
    RSH,

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
    pub fn as_str(&self) -> String {
        match &self {
            OpCode::ADD => String::from("ADD"),
            OpCode::SUB => String::from("SUB"),
            OpCode::MUL => String::from("MUL"),
            OpCode::DIV => String::from("DIV"),
            OpCode::MOD => String::from("MOD"),
            
            OpCode::NOT => String::from("NOT"),
            OpCode::AND => String::from("AND"),
            OpCode::ORR => String::from("ORR"),
            OpCode::XOR => String::from("XOR"),
            OpCode::LSH => String::from("LSH"),
            OpCode::RSH => String::from("RSH"),

            OpCode::LDR => String::from("LDR"),
            OpCode::SET => String::from("SET"),
            OpCode::RND => String::from("RND"),
            OpCode::PSH => String::from("PSH"),
            OpCode::POP => String::from("POP"),

            OpCode::BRU => String::from("BRU"),
            OpCode::BRZ => String::from("BNZ"),
            OpCode::BRN => String::from("BRN"),
            OpCode::BRP => String::from("BRP"),
            OpCode::FUN => String::from("FUN"),
            OpCode::RET => String::from("RET"),

            OpCode::HCF => String::from("HCF")
        }
    }

    pub fn as_i8(&self) -> i8 {
        match &self {
            OpCode::ADD => 0x00,
            OpCode::SUB => 0x01,
            OpCode::MUL => 0x02,
            OpCode::DIV => 0x03,
            OpCode::MOD => 0x04,
            
            OpCode::NOT => 0x05,
            OpCode::AND => 0x06,
            OpCode::ORR => 0x07,
            OpCode::XOR => 0x08,
            OpCode::LSH => 0x08,
            OpCode::RSH => 0x08,

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

