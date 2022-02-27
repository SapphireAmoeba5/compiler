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

    let mut block_id: usize = 0;
    let mut block_stack: Vec<(usize, Operation)> = Vec::new();

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

            "dup" => tokens.push(Instruction::new(Operation::Dupe, None)),
            "pop" => tokens.push(Instruction::new(Operation::Pop, None)),
            "swap" => tokens.push(Instruction::new(Operation::Swap, None)),
            "over" => tokens.push(Instruction::new(Operation::Over, None)),
            "rot" => tokens.push(Instruction::new(Operation::Rot, None)),
            "if" => {
                // Increment block_id to get a unique label name for each label
                block_id += 1;
                tokens.push(Instruction::new(
                    Operation::If,
                    Some(vec![block_id.to_string()]),
                ));
                block_stack.push((block_id, Operation::If));
            }
            "while" => {
                block_id += 1;
                tokens.push(Instruction::new(
                    Operation::While,
                    Some(vec![block_id.to_string()]),
                ));
                block_stack.push((block_id, Operation::While));
            }
            "do" => {
                let id = match block_stack.last() {
                    Some(_) => {
                        block_id += 1;
                        block_id
                    }
                    None => {
                        error_println!("'do' statement without matching while statement");
                        return Err(());
                    }
                };
                tokens.push(Instruction::new(Operation::Do, Some(vec![id.to_string()])));
            }
            "else" => {
                let id = match block_stack.pop() {
                    Some(tmp_id) => {
                        // Increment block_id for label pointing to end of if-else block & replace the id at the top of the block_stack with it
                        block_id += 1;
                        block_stack.push((block_id, Operation::If));
                        tmp_id
                    }
                    None => {
                        error_println!("Homeless 'else'");
                        return Err(());
                    }
                };
                // Push the label pointing to the end of the block & the label name for label that points to the else code
                tokens.push(Instruction::new(
                    Operation::Else,
                    Some(vec![block_id.to_string(), id.0.to_string()]),
                ));
            }
            "end" => {
                let (id, operation) = match block_stack.pop() {
                    Some(t) => t,
                    None => {
                        error_println!("Homeless 'end'");
                        return Err(());
                    }
                };
                let operation = match operation {
                    Operation::If => "if",
                    Operation::While => "while",
                    _ => "null",
                };

                tokens.push(Instruction::new(
                    Operation::End,
                    Some(vec![id.to_string(), operation.to_string()]),
                ));
            }
            _ => {
                match instr.parse::<u64>() {
                    Ok(_) => {}
                    Err(_) => {
                        error_println!("{} is not recognizable syntax", instr);
                        return Err(());
                    }
                }
                tokens.push(Instruction::new(
                    Operation::Push,
                    Some(vec![instr.to_string()]),
                ));
            }
        }
    }

    if !block_stack.is_empty() {
        error_println!("Block missing matching end statement");
        return Err(());
    }

    Ok(tokens)
}
