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
            "." => tokens.push(Instruction::new(
                Operation::Dump,
                String::new(),
                instr.column,
                instr.line_number,
            )),

            "+" => tokens.push(Instruction::new(
                Operation::Add,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "-" => tokens.push(Instruction::new(
                Operation::Sub,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "*" => tokens.push(Instruction::new(
                Operation::Mul,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "/" => tokens.push(Instruction::new(
                Operation::Div,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "%" => tokens.push(Instruction::new(
                Operation::Mod,
                String::new(),
                instr.column,
                instr.line_number,
            )),

            "=" => tokens.push(Instruction::new(
                Operation::Eq,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            ">" => tokens.push(Instruction::new(
                Operation::GreaterThan,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            ">=" => tokens.push(Instruction::new(
                Operation::GreaterThanEqual,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "<" => tokens.push(Instruction::new(
                Operation::LessThan,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "<=" => tokens.push(Instruction::new(
                Operation::LessThanEqual,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "!" => tokens.push(Instruction::new(
                Operation::Not,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "&&" => tokens.push(Instruction::new(
                Operation::And,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "||" => tokens.push(Instruction::new(
                Operation::Or,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "~" => tokens.push(Instruction::new(
                Operation::BitwiseNot,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "&" => tokens.push(Instruction::new(
                Operation::BitwiseAnd,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "|" => tokens.push(Instruction::new(
                Operation::BitwiseOr,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "^" => tokens.push(Instruction::new(
                Operation::BitwiseXor,
                String::new(),
                instr.column,
                instr.line_number,
            )),

            "dup" => tokens.push(Instruction::new(
                Operation::Dupe,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "pop" => tokens.push(Instruction::new(
                Operation::Pop,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "swap" => tokens.push(Instruction::new(
                Operation::Swap,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "over" => tokens.push(Instruction::new(
                Operation::Over,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "rot" => tokens.push(Instruction::new(
                Operation::Rot,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "putc" => tokens.push(Instruction::new(
                Operation::Putc,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "puts" => tokens.push(Instruction::new(
                Operation::Puts,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "strlen" => tokens.push(Instruction::new(
                Operation::Strlen,
                String::new(),
                instr.column,
                instr.line_number,
            )),

            "func" => tokens.push(Instruction::new(
                Operation::Func,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "->" => tokens.push(Instruction::new(
                Operation::Arrow,
                String::new(),
                instr.column,
                instr.line_number,
            )),
            "in" => tokens.push(Instruction::new(
                Operation::In,
                String::new(),
                instr.column,
                instr.line_number,
            )),

            "if" => {
                tokens.push(Instruction::new(
                    Operation::If,
                    String::new(),
                    instr.column,
                    instr.line_number,
                ));
            }
            "while" => {
                tokens.push(Instruction::new(
                    Operation::While,
                    String::new(),
                    instr.column,
                    instr.line_number,
                ));
            }
            "do" => {
                tokens.push(Instruction::new(
                    Operation::Do,
                    String::new(),
                    instr.column,
                    instr.line_number,
                ));
            }
            "else" => {
                tokens.push(Instruction::new(
                    Operation::Else,
                    String::new(),
                    instr.column,
                    instr.line_number,
                ));
            }
            "end" => {
                tokens.push(Instruction::new(
                    Operation::End,
                    String::new(),
                    instr.column,
                    instr.line_number,
                ));
            }
            _ => {
                match try_parse_number(&instr) {
                    Ok(s) => {
                        tokens.push(Instruction::new(
                            Operation::Push,
                            s,
                            instr.column,
                            instr.line_number,
                        ));
                        continue;
                    }
                    Err(_) => {}
                }

                match try_parse_string(&instr) {
                    Ok(s) => {
                        tokens.push(Instruction::new(
                            Operation::PushString,
                            s,
                            instr.column,
                            instr.line_number,
                        ));
                        continue;
                    }
                    Err(_) => {}
                }

                match try_parse_char(&instr) {
                    Ok(s) => {
                        tokens.push(Instruction::new(
                            Operation::Push,
                            s.to_string(),
                            instr.column,
                            instr.line_number,
                        ));
                        continue;
                    }
                    Err(_) => {}
                }

                match try_parse_identifier(&instr) {
                    Ok(s) => {
                        tokens.push(Instruction::new(
                            Operation::Identifier,
                            s,
                            instr.column,
                            instr.line_number,
                        ));
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
