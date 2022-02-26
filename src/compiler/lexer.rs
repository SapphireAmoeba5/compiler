use crate::{error_println, operation::*};

use std::mem::replace;

pub fn lex_source(source: &str) -> Result<Vec<Instruction>, ()> {
    let source: Vec<&str> = source.split_ascii_whitespace().collect();
    let mut tokens: Vec<Instruction> = Vec::new();

    let mut block_id: usize = 0;
    let mut block_stack: Vec<usize> = Vec::new();

    for instr in source.iter() {
        match *instr {
            "." => tokens.push(Instruction::new(Operation::Dump, None)),

            "+" => tokens.push(Instruction::new(Operation::Add, None)),
            "-" => tokens.push(Instruction::new(Operation::Sub, None)),
            "*" => tokens.push(Instruction::new(Operation::Mul, None)),
            "/" => tokens.push(Instruction::new(Operation::Div, None)),
            "%" => tokens.push(Instruction::new(Operation::Mod, None)),

            "=" => tokens.push(Instruction::new(Operation::Eq, None)),
            ">" => tokens.push(Instruction::new(Operation::GreaterThan, None)),
            ">=" => tokens.push(Instruction::new(Operation::GreaterThanEqual, None)),
            "<" => tokens.push(Instruction::new(Operation::LessThan, None)),
            "<=" => tokens.push(Instruction::new(Operation::LessThanEqual, None)),
            "!" => tokens.push(Instruction::new(Operation::Not, None)),

            "~" => tokens.push(Instruction::new(Operation::BitwiseNot, None)),
            "&" => tokens.push(Instruction::new(Operation::BitwiseAnd, None)),
            "|" => tokens.push(Instruction::new(Operation::BitwiseOr, None)),
            "^" => tokens.push(Instruction::new(Operation::BitwiseXor, None)),

            "if" => {
                tokens.push(Instruction::new(Operation::If, Some(block_id.to_string())));
                block_stack.push(block_id);
                block_id += 1;
            }
            "else" => {
                let id = match block_stack.last() {
                    Some(t) => {
                        let ret = t.clone();
                        replace(block_stack.last_mut().unwrap(), block_id);
                        ret
                    }
                    None => {
                        error_println!("Homeless 'else'");
                        return Err(());
                    }
                };
                tokens.push(Instruction::new(Operation::Else, Some(id.to_string())));
            }
            "end" => {
                let id = match block_stack.pop() {
                    Some(t) => t,
                    None => {
                        error_println!("Homeless 'end'");
                        return Err(());
                    }
                };
                tokens.push(Instruction::new(Operation::End, Some(id.to_string())));
            }
            _ => {
                match instr.parse::<u64>() {
                    Ok(_) => {}
                    Err(_) => {
                        error_println!("{} is not recognizable syntax", instr);
                        return Err(());
                    }
                }
                tokens.push(Instruction::new(Operation::Push, Some(instr.to_string())));
            }
        }
    }

    if !block_stack.is_empty() {
        error_println!("Block missing matching end statement");
        return Err(());
    }

    Ok(tokens)
}
