mod split;

use crate::{debug_println, error_println, info_println, operation::*, warn_println};
use split::*;

use std::{fmt::Debug, mem::replace};

fn try_parse_number(word: &Token) -> Result<String, ()> {
    let number = word.token.as_str();

    match number.parse::<u64>() {
        Ok(_) => {
            return Ok(number.to_string());
        }
        Err(_) => {}
    }

    match number.parse::<i64>() {
        Ok(_) => {
            warn_println!(
                "This language currently doesn't support signed values {}:{}",
                word.line_number,
                word.column
            );
            return Ok(number.to_string());
        }
        Err(_) => {}
    }

    match &number[..2] {
        "0x" => {
            if number.len() - 2 > 16 {
                error_println!(
                    "Hexadecimal literal too large {}:{}",
                    word.line_number,
                    word.column
                );
                return Err(());
            }

            match u64::from_str_radix(&number[2..], 16) {
                Ok(s) => {
                    return Ok(s.to_string());
                }
                Err(_) => {
                    error_println!(
                        "Invalid hexadecimal literal {}:{}",
                        word.line_number,
                        word.column
                    );
                    return Err(());
                }
            }
        }
        "0b" => {
            if number.len() - 2 > 64 {
                error_println!(
                    "Binary literal too large {}:{}",
                    word.line_number,
                    word.column
                );
                return Err(());
            }

            match u64::from_str_radix(&number[2..], 2) {
                Ok(s) => {
                    return Ok(s.to_string());
                }
                Err(_) => {
                    error_println!(
                        "Invalid binary literal {}:{}",
                        word.line_number,
                        word.column
                    );
                    return Err(());
                }
            }
        }

        _ => {}
    }

    Err(())
}

fn try_parse_string(word: &Token) -> Result<String, ()> {
    let string = word.token.as_str();

    if string.starts_with('"') && string.ends_with('"') {
        let mut escaped = false;

        for (i, ch) in string[1..].chars().enumerate() {
            if ch == '"' && !escaped {
                if string[..=i].len() + 1 != string.len() {
                    return Err(());
                }
                return Ok(string.to_string());
            }

            escaped = ch == '\\'
        }
    }

    Err(())
}

pub fn lex_source(source: &str) -> Result<Vec<Instruction>, ()> {
    // TODO: Create own split function that stores the line and column each token occured on
    let words: Vec<Token> = split_tokens(source);

    let mut tokens: Vec<Instruction> = Vec::new();

    for instr in words.iter() {
        match instr.token.as_str() {
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
                match try_parse_number(&instr) {
                    Ok(s) => {
                        tokens.push(Instruction::new(Operation::Push, s));
                    }
                    Err(_) => {}
                }

                match try_parse_string(&instr) {
                    Ok(s) => {
                        debug_println!("Parsed string: {}", s);
                        tokens.push(Instruction::new(Operation::PushString, s));
                    }
                    Err(_) => {}
                }
                // tokens.push(Instruction::new(Operation::Push, instr.to_string()));
            }
        }
    }

    Ok(tokens)
}
