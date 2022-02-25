mod compiler;
mod interpreter;
mod logger;
mod operation;

use std::time::Instant;

use compiler::*;
use interpreter::*;

// TODO: Remove this after debugging
use operation::*;

fn main() {
    let tokens: Vec<Instruction> = vec![
        Instruction::new(Operation::Push, Some("150".to_string())),
        Instruction::new(Operation::Push, Some("150".to_string())),
        Instruction::new(Operation::Add, None),
        Instruction::new(Operation::Dump, None),
    ];

    let s = Compiler::new("nothing", "output.asm");
    s.unwrap().output_to_assembly(&tokens);
}
