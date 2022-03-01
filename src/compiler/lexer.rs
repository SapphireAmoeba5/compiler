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
            "." => tokens.push(Instruction::new(Operation::Dump, Vec::new())),

            "+" => tokens.push(Instruction::new(Operation::Add, Vec::new())),
            "-" => tokens.push(Instruction::new(Operation::Sub, Vec::new())),
            "*" => tokens.push(Instruction::new(Operation::Mul, Vec::new())),
            "/" => tokens.push(Instruction::new(Operation::Div, Vec::new())),
            "%" => tokens.push(Instruction::new(Operation::Mod, Vec::new())),

            "=" => tokens.push(Instruction::new(Operation::Eq, Vec::new())),
            ">" => tokens.push(Instruction::new(Operation::GreaterThan, Vec::new())),
            ">=" => tokens.push(Instruction::new(Operation::GreaterThanEqual, Vec::new())),
            "<" => tokens.push(Instruction::new(Operation::LessThan, Vec::new())),
            "<=" => tokens.push(Instruction::new(Operation::LessThanEqual, Vec::new())),
            "!" => tokens.push(Instruction::new(Operation::Not, Vec::new())),
            "&&" => tokens.push(Instruction::new(Operation::And, Vec::new())),
            "||" => tokens.push(Instruction::new(Operation::Or, Vec::new())),
            "~" => tokens.push(Instruction::new(Operation::BitwiseNot, Vec::new())),
            "&" => tokens.push(Instruction::new(Operation::BitwiseAnd, Vec::new())),
            "|" => tokens.push(Instruction::new(Operation::BitwiseOr, Vec::new())),
            "^" => tokens.push(Instruction::new(Operation::BitwiseXor, Vec::new())),

            "dup" => tokens.push(Instruction::new(Operation::Dupe, Vec::new())),
            "pop" => tokens.push(Instruction::new(Operation::Pop, Vec::new())),
            "swap" => tokens.push(Instruction::new(Operation::Swap, Vec::new())),
            "over" => tokens.push(Instruction::new(Operation::Over, Vec::new())),
            "rot" => tokens.push(Instruction::new(Operation::Rot, Vec::new())),
            "if" => {
                // Increment block_id to get a unique label name for each label
                block_id += 1;
                tokens.push(Instruction::new(Operation::If, vec![block_id.to_string()]));
                block_stack.push((block_id, Operation::If));
            }
            "while" => {
                block_id += 1;
                tokens.push(Instruction::new(
                    Operation::While,
                    vec![block_id.to_string()],
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
                tokens.push(Instruction::new(Operation::Do, vec![id.to_string()]));
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
                    vec![block_id.to_string(), id.0.to_string()],
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
                    vec![id.to_string(), operation.to_string()],
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
                tokens.push(Instruction::new(Operation::Push, vec![instr.to_string()]));
            }
        }
    }

    if !block_stack.is_empty() {
        error_println!("Block missing matching end statement");
        return Err(());
    }

    Ok(tokens)
}
