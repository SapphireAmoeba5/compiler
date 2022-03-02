use crate::{error_println, operation::*};

use std::mem::replace;

struct Token {
    token: String,
    line_number: usize,
    column: usize,
}

pub fn lex_source(source: &str) -> Result<Vec<Instruction>, ()> {
    // TODO: Create own split function that stores the line and column each token occured on
    let source: Vec<&str> = source.split_ascii_whitespace().collect();

    let mut tokens: Vec<Instruction> = Vec::new();

    for instr in source.iter() {
        match *instr {
            "." => tokens.push(Instruction::new(Operation::Dump, String::new())),

            "+" => tokens.push(Instruction::new(Operation::Add, String::new())),
            "-" => tokens.push(Instruction::new(Operation::Sub, String::new())),
            "*" => tokens.push(Instruction::new(Operation::Mul, String::new())),
            "/" => tokens.push(Instruction::new(Operation::Div, String::new())),
            "%" => tokens.push(Instruction::new(Operation::Mod, String::new())),

            "=" => tokens.push(Instruction::new(Operation::Eq, String::new())),
            ">" => tokens.push(Instruction::new(Operation::GreaterThan, String::new())),
            ">=" => tokens.push(Instruction::new(Operation::GreaterThanEqual, String::new())),
            "<" => tokens.push(Instruction::new(Operation::LessThan, String::new())),
            "<=" => tokens.push(Instruction::new(Operation::LessThanEqual, String::new())),
            "!" => tokens.push(Instruction::new(Operation::Not, String::new())),
            "&&" => tokens.push(Instruction::new(Operation::And, String::new())),
            "||" => tokens.push(Instruction::new(Operation::Or, String::new())),
            "~" => tokens.push(Instruction::new(Operation::BitwiseNot, String::new())),
            "&" => tokens.push(Instruction::new(Operation::BitwiseAnd, String::new())),
            "|" => tokens.push(Instruction::new(Operation::BitwiseOr, String::new())),
            "^" => tokens.push(Instruction::new(Operation::BitwiseXor, String::new())),

            "dup" => tokens.push(Instruction::new(Operation::Dupe, String::new())),
            "pop" => tokens.push(Instruction::new(Operation::Pop, String::new())),
            "swap" => tokens.push(Instruction::new(Operation::Swap, String::new())),
            "over" => tokens.push(Instruction::new(Operation::Over, String::new())),
            "rot" => tokens.push(Instruction::new(Operation::Rot, String::new())),
            "if" => {
                tokens.push(Instruction::new(Operation::If, String::new()));
            }
            "while" => {
                tokens.push(Instruction::new(Operation::While, String::new()));
            }
            "do" => {
                tokens.push(Instruction::new(Operation::Do, String::new()));
            }
            "else" => {
                tokens.push(Instruction::new(Operation::Else, String::new()));
            }
            "end" => {
                tokens.push(Instruction::new(Operation::End, String::new()));
            }
            _ => {
                tokens.push(Instruction::new(Operation::Push, instr.to_string()));
            }
        }
    }

    Ok(tokens)
}
