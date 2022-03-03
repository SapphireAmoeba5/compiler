mod split;
mod try_parse;

use crate::{debug_println, error_println, info_println, operation::*, warn_println};
use split::*;
use try_parse::*;

use std::{fmt::Debug, mem::replace};

pub fn lex_source(source: &str) -> Result<Vec<Instruction>, ()> {
    let words: Vec<Token> = split_tokens(source);
    let mut failed = false;
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
            "putc" => tokens.push(Instruction::new(Operation::Putc, String::new())),
            "puts" => tokens.push(Instruction::new(Operation::Puts, String::new())),
            "strlen" => tokens.push(Instruction::new(Operation::Strlen, String::new())),

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
                        continue;
                    }
                    Err(_) => {}
                }

                match try_parse_string(&instr) {
                    Ok(s) => {
                        tokens.push(Instruction::new(Operation::PushString, s));
                        continue;
                    }
                    Err(_) => {}
                }
                failed = true;
                error_println!(
                    "{} is an unknown token {}:{}",
                    instr.token,
                    instr.line_number,
                    instr.column
                );
            }
        }
    }

    if !failed {
        Ok(tokens)
    } else {
        Err(())
    }
}
