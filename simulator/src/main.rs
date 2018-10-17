extern crate definitions;

use definitions::op_code::OpCode;

fn main() { 
    let val = OpCode::ADD;

    if let OpCode::ADD = val {
        println!("Woohoo!");
    }
    
    if OpCode::ADD == val {
        println!("Woohoo too! {} {}", val.as_str(), val.as_i8());
    }
}

