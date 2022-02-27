mod asm_generate;
mod lexer;

use std::fs::File;
use std::io::Write;
use std::iter::Inspect;
use std::time::Instant;

use crate::operation::*;
use crate::{debug_println, error_println, info_println, info_println_if};
use asm_generate::*;
use lexer::*;
pub struct Compiler {
    input_file: String,
    output_file: String,
}

impl Compiler {
    pub fn new(input_file: &str, output_file: &str) -> Result<Self, ()> {
        let mut s = Self {
            input_file: input_file.to_string(),
            output_file: output_file.to_string(),
        };

        let source = match std::fs::read_to_string(&s.input_file) {
            Ok(s) => s,
            Err(e) => {
                error_println!("Error opening input file :: Error: {}", e);
                return Err(());
            }
        };

        let start = Instant::now();

        let lexed = match lex_source(&source) {
            Ok(t) => t,
            Err(_) => {
                return Err(());
            }
        };

        s.output_to_assembly(&lexed);

        info_println!(
            "Compilation finished! {}s ({}ms)",
            start.elapsed().as_secs_f64(),
            start.elapsed().as_millis()
        );

        Ok(s)
    }
}

impl Compiler {
    /// Converts and outputs tokens into assembly
    fn output_to_assembly(&mut self, tokens: &Vec<Instruction>) {
        let mut assembly: String = String::new();

        initialize_asm(&mut assembly);

        for instr in tokens.iter() {
            let asm: String;
            match instr.op {
                Operation::Push => {
                    asm = asm_push(instr);
                }
                Operation::Dump => {
                    asm = asm_dump(instr);
                }
                Operation::Dupe => {
                    asm = asm_dupe(instr);
                }
                Operation::Pop => {
                    asm = asm_pop(instr);
                }
                Operation::Swap => {
                    asm = asm_swap(instr);
                }
                Operation::Over => {
                    asm = asm_over(instr);
                }
                Operation::Rot => {
                    asm = asm_rot(instr);
                }
                Operation::Add => {
                    asm = asm_add(instr);
                }
                Operation::Sub => {
                    asm = asm_sub(instr);
                }
                Operation::Mul => {
                    asm = asm_mul(instr);
                }
                Operation::Div => {
                    asm = asm_div(instr);
                }
                Operation::Mod => {
                    asm = asm_mod(instr);
                }
                Operation::Eq => {
                    asm = asm_eq(instr);
                }
                Operation::GreaterThan => {
                    asm = asm_greater_than(instr);
                }
                Operation::GreaterThanEqual => {
                    asm = asm_greater_than_eq(instr);
                }
                Operation::LessThan => {
                    asm = asm_less_than(instr);
                }
                Operation::LessThanEqual => {
                    asm = asm_less_than_eq(instr);
                }
                Operation::Not => {
                    asm = asm_not(instr);
                }
                Operation::BitwiseNot => {
                    asm = asm_bitwise_not(instr);
                }
                Operation::BitwiseAnd => {
                    asm = asm_bitwise_and(instr);
                }
                Operation::BitwiseOr => {
                    asm = asm_bitwise_or(instr);
                }
                Operation::BitwiseXor => {
                    asm = asm_bitwise_xor(instr);
                }
                Operation::If => {
                    asm = asm_if(instr);
                }
                Operation::While => {
                    asm = asm_while(instr);
                }
                Operation::Do => {
                    asm = asm_do(instr);
                }
                Operation::Else => {
                    asm = asm_else(instr);
                }
                Operation::End => {
                    asm = asm_end(instr);
                }
                _ => {
                    asm = String::new();
                }
            }
            assembly.push_str(&asm);
        }

        // Push code to return from program
        assembly.push_str(
            "    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall",
        );

        // Output generated assembly to file
        let mut out_file = match File::create(&self.output_file) {
            Ok(s) => s,
            Err(e) => {
                error_println!("Error opening output file :: Error: {}", e);
                return;
            }
        };

        match out_file.write(assembly.as_bytes()) {
            Ok(s) => {}
            _ => {
                error_println!("Unknown error writing to output file");
            }
        }
    }
}
