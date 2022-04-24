use std::collections::HashMap;

use crate::compiler::lexer::*;
use crate::operation::*;
use crate::{debug_println, error_println, info_println, info_println_if};

pub struct Interpreter {
    input_file: String,
    stack: Vec<u64>,

    strings: Vec<String>,
    string_map: HashMap<String, u64>,

    instructions: Vec<Instruction>,
}

impl Interpreter {
    pub fn new(input_file: &str) -> Self {
        Self {
            input_file: input_file.to_string(),
            stack: Vec::new(),

            strings: Vec::new(),
            string_map: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), ()> {
        let source = match std::fs::read_to_string(&self.input_file) {
            Ok(s) => s,
            Err(e) => {
                error_println!("Error opening input file :: Error: {}", e);
                return Err(());
            }
        };

        self.instructions = lex_source(&source)?;
        self.preprocess_instructions()?;
        self.interpret_instructions()?;

        Ok(())
    }
}

impl Interpreter {
    fn interpret_instructions(&mut self) -> Result<(), ()> {
        // Iterate vector with index variable to make it easier to implement control flow statements
        let mut i = 0;
        while i < self.instructions.len() {
            // TODO: remove this clone
            let instr = self.instructions[i].clone();
            match instr.op {
                Operation::Push => {
                    self.op_push(&instr)?;
                }
                Operation::PushString => {
                    self.op_push_string(&instr)?;
                }
                Operation::Dump => {
                    self.op_dump(&instr)?;
                }
                Operation::Dupe => {
                    self.op_dupe(&instr)?;
                }
                Operation::Pop => {
                    self.op_pop(&instr)?;
                }
                Operation::Swap => {
                    self.op_swap(&instr)?;
                }
                Operation::Over => {
                    self.op_over(&instr)?;
                }
                Operation::Rot => {
                    self.op_rot(&instr)?;
                }
                Operation::Putc => {
                    self.op_putc(&instr)?;
                }
                Operation::Puts => {
                    self.op_puts(&instr)?;
                }
                Operation::Strlen => {
                    self.op_strlen(&instr)?;
                }
                Operation::Add => {
                    self.op_add(&instr)?;
                }
                Operation::Sub => {
                    self.op_sub(&instr)?;
                }
                Operation::Mul => {
                    self.op_mul(&instr)?;
                }
                Operation::Div => {
                    self.op_div(&instr)?;
                }
                Operation::Mod => {
                    self.op_mod(&instr)?;
                }
                Operation::Eq => {
                    self.op_eq(&instr)?;
                }
                Operation::GreaterThan => {
                    self.op_greater_than(&instr)?;
                }
                Operation::GreaterThanEqual => {
                    self.op_greater_than_eq(&instr)?;
                }
                Operation::LessThan => {
                    self.op_less_than(&instr)?;
                }
                Operation::LessThanEqual => {
                    self.op_less_than_eq(&instr)?;
                }
                Operation::Not => {
                    self.op_not(&instr)?;
                }
                Operation::And => {
                    self.op_and(&instr)?;
                }
                Operation::Or => {
                    self.op_or(&instr)?;
                }
                Operation::BitwiseNot => {
                    self.op_bitwise_not(&instr)?;
                }
                Operation::BitwiseAnd => {
                    self.op_bitwise_and(&instr)?;
                }
                Operation::BitwiseOr => {
                    self.op_bitwise_or(&instr)?;
                }
                Operation::BitwiseXor => {
                    self.op_bitwise_xor(&instr)?;
                }

                Operation::If => {
                    self.op_if(&instr, &mut i)?;
                }
                Operation::Else => {
                    self.op_else(&instr, &mut i)?;
                }
                Operation::While => {}
                Operation::Do => {
                    self.op_do(&instr, &mut i)?;
                }
                Operation::End => {
                    self.op_end(&instr, &mut i)?;
                }

                _ => {
                    error_println!(
                        "Runtime error: Unknown instruction ecountered {}:{}",
                        instr.line_number,
                        instr.column
                    );
                    return Err(());
                }
            }
            i += 1;
        }

        Ok(())
    }

    fn preprocess_instructions(&mut self) -> Result<(), ()> {
        let mut current_function: Option<String> = None;
        let mut block_stack: Vec<(Operation, usize)> = Vec::new();

        for i in 0..self.instructions.len() {
            let cur = &self.instructions[i];
            match cur.op {
                Operation::If => block_stack.push((Operation::If, i)),
                Operation::While => block_stack.push((Operation::While, i)),
                Operation::Do => {
                    if let Some(s) = block_stack.last() {
                        if s.0 != Operation::While {
                            error_println!(
                                "Do statement requires while statement {}:{}",
                                cur.line_number,
                                cur.column
                            );
                            return Err(());
                        }
                    } else {
                        error_println!(
                            "Do statement requires while statement {}:{}",
                            cur.line_number,
                            cur.column
                        );
                        return Err(());
                    }
                    block_stack.push((Operation::Do, i))
                }

                Operation::Else => {
                    if let Some(s) = block_stack.pop() {
                        if s.0 != Operation::If {
                            error_println!(
                                "Else statement must be connected to an if statement {}:{}",
                                cur.line_number,
                                cur.column
                            );
                            return Err(());
                        }

                        self.instructions[s.1].value = i.to_string();
                        block_stack.push((Operation::If, i));
                    } else {
                        error_println!(
                            "Else statement must be connected to an if statement {}:{}",
                            cur.line_number,
                            cur.column
                        );
                        return Err(());
                    }
                }
                Operation::End => match block_stack.pop() {
                    Some(s) => match s.0 {
                        Operation::If => {
                            self.instructions[s.1].value = i.to_string();
                        }
                        Operation::Do => {
                            // Safe to unwrap because this is validated elsewhere
                            let while_id = block_stack.pop().unwrap();
                            self.instructions[i].value = while_id.1.to_string();
                            self.instructions[s.1].value = i.to_string();
                        }
                        // This is here just to print a different error message
                        Operation::While => {
                            error_println!(
                                "While loop requires do statement before next end statement {}:{}",
                                cur.line_number,
                                cur.column
                            );
                            return Err(());
                        }
                        _ => {
                            error_println!(
                                "End statement requires block statement {}:{}",
                                cur.line_number,
                                cur.column
                            );
                            return Err(());
                        }
                    },
                    None => {
                        error_println!(
                            "Unmatched end statement {}:{}",
                            cur.line_number,
                            cur.column
                        );
                        return Err(());
                    }
                },
                _ => {}
            }
        }

        if block_stack.is_empty() == false {
            error_println!("Missing end statement");
            return Err(());
        }

        Ok(())
    }

    fn char_to_escape_code(ch: char) -> Result<u8, ()> {
        match ch {
            '\'' => Ok(0x27),
            '"' => Ok(0x22),
            '?' => Ok(0x3f),
            '\\' => Ok(0x5c),
            'a' => Ok(0x07),
            'b' => Ok(0x08),
            'f' => Ok(0x0c),
            'n' => Ok(0x0a),
            'r' => Ok(0x0d),
            't' => Ok(0x09),
            'v' => Ok(0x0b),
            '0' => Ok(0x00),
            _ => {
                error_println!("\\{} is not a valid escape character", ch);
                Err(())
            }
        }
    }

    fn parse_string(string: &str) -> Result<String, ()> {
        let mut parsed = String::new();
        let mut escaped = false;
        for ch in string.chars() {
            if ch == '\\' && !escaped {
                escaped = true;
            } else if !escaped {
                parsed.push(ch);
            } else if escaped {
                escaped = false;
                let escape_ch = Interpreter::char_to_escape_code(ch)? as char;
                parsed.push(escape_ch);
            }
        }

        Ok(parsed)
    }
}

impl Interpreter {
    fn op_push(&mut self, instr: &Instruction) -> Result<(), ()> {
        self.stack.push(instr.value.parse::<u64>().unwrap());
        Ok(())
    }

    fn op_push_string(&mut self, instr: &Instruction) -> Result<(), ()> {
        let string = Interpreter::parse_string(&instr.value[1..instr.value.len() - 1])?;
        let loc = match self.string_map.get(&string) {
            Some(s) => s.clone(),
            None => {
                // Strip off the leading and trailing " character
                let index = self.strings.len();
                self.string_map.insert(string.to_string(), index as u64);
                self.strings.push(string.to_string());
                index as u64
            }
        };
        self.stack.push(loc);
        Ok(())
    }

    fn op_swap(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'swap' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'swap' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val0);
        self.stack.push(val1);

        Ok(())
    }

    fn op_over(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val = match self.stack.get(self.stack.len() - 2) {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'over' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };
        self.stack.push(val);
        Ok(())
    }

    fn op_rot(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'rot' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'rot' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val2 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'rot' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val1);
        self.stack.push(val0);
        self.stack.push(val2);

        Ok(())
    }

    fn op_putc(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'putc' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        print!("{}", val as u8 as char);
        Ok(())
    }

    fn op_puts(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'puts' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let string = match self.strings.get(val as usize) {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'puts' but '{}' is not a valid string ID {}:{}",
                    val,
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };
        print!("{}", string);
        Ok(())
    }

    fn op_strlen(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'strlen' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let string = match self.strings.get(val as usize) {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'strlen' but '{}' is not a valid string ID {}:{}",
                    instr.line_number,
                    instr.column,
                    val
                );
                return Err(());
            }
        };

        self.stack.push(string.len() as u64);
        Ok(())
    }

    fn op_add(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'op_add' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'op_add' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val0 + val1);
        Ok(())
    }

    fn op_mul(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'multiply' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'multiply' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val0 * val1);
        Ok(())
    }

    fn op_div(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'divide' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'divide' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val1 / val0);
        Ok(())
    }

    fn op_mod(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'modulus' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'modulus' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val1 % val0);
        Ok(())
    }

    fn op_greater_than(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called '>' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called '>' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push((val1 > val0) as u64);
        Ok(())
    }

    fn op_greater_than_eq(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called '>=' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called '>=' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push((val1 >= val0) as u64);
        Ok(())
    }

    fn op_less_than(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called '<' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called '<' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push((val1 < val0) as u64);
        Ok(())
    }

    fn op_less_than_eq(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called '<=' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called '<=' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push((val1 <= val0) as u64);
        Ok(())
    }

    fn op_not(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'not' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push((val == 0) as u64);
        Ok(())
    }

    fn op_and(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'and' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'and' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push((val1 != 0 && val0 != 0) as u64);
        Ok(())
    }

    fn op_or(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'or' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'or' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push((val1 != 0 || val0 != 0) as u64);
        Ok(())
    }

    fn op_bitwise_not(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'bitwise not' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };
        self.stack.push(!val);
        Ok(())
    }

    fn op_bitwise_and(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'bitwise and' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'bitwise and' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val0 & val1);
        Ok(())
    }

    fn op_bitwise_or(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'bitwise or' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'bitwise or' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val0 | val1);
        Ok(())
    }

    fn op_bitwise_xor(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'bitwise xor' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'bitwise xor' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push(val0 ^ val1);
        Ok(())
    }

    fn op_dump(&mut self, instr: &Instruction) -> Result<(), ()> {
        match self.stack.pop() {
            Some(s) => {
                println!("{}", s);
            }
            None => {
                error_println!(
                    "Runtime error: Called 'dump' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        }
        Ok(())
    }

    fn op_if(&mut self, instr: &Instruction, i: &mut usize) -> Result<(), ()> {
        let val = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'if' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };
        if val == 0 {
            *i = instr.value.parse().unwrap();
        }
        Ok(())
    }

    fn op_else(&mut self, instr: &Instruction, i: &mut usize) -> Result<(), ()> {
        *i = instr.value.parse().unwrap();
        Ok(())
    }

    fn op_do(&mut self, instr: &Instruction, i: &mut usize) -> Result<(), ()> {
        let val = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'do' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        if val == 0 {
            *i = instr.value.parse().unwrap();
        }
        Ok(())
    }

    fn op_end(&mut self, instr: &Instruction, i: &mut usize) -> Result<(), ()> {
        if !instr.value.is_empty() {
            *i = instr.value.parse().unwrap();
        }
        Ok(())
    }

    fn op_sub(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'subtract' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s,
            None => {
                error_println!(
                    "Runtime error: Called 'subtract' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let dif = val1.wrapping_sub(val0);
        self.stack.push(dif);
        Ok(())
    }

    fn op_dupe(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val = match self.stack.last() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'dupe' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };
        self.stack.push(val);
        Ok(())
    }

    fn op_eq(&mut self, instr: &Instruction) -> Result<(), ()> {
        let val0 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'eq' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        let val1 = match self.stack.pop() {
            Some(s) => s.clone(),
            None => {
                error_println!(
                    "Runtime error: Called 'eq' but stack was empty {}:{}",
                    instr.line_number,
                    instr.column
                );
                return Err(());
            }
        };

        self.stack.push((val0 == val1) as u64);
        Ok(())
    }

    fn op_pop(&mut self, instr: &Instruction) -> Result<(), ()> {
        if self.stack.pop().is_none() {
            error_println!(
                "Runtime error: Called 'pop' but stack was empty {}:{}",
                instr.line_number,
                instr.column
            );
            return Err(());
        }
        Ok(())
    }
}
