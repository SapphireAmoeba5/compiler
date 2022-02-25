mod asm_generate;

use std::fs::File;
use std::io::Write;

use crate::operation::*;
use crate::{debug_println, error_println, info_println, info_println_if};
use asm_generate::*;
pub struct Compiler {
    input_file: String,
    output_file: String,
}

impl Compiler {
    pub fn new(input_file: &str, output_file: &str) -> Result<Self, ()> {
        let s = Self {
            input_file: input_file.to_string(),
            output_file: output_file.to_string(),
        };

        Ok(s)
    }
}

impl Compiler {
    // TODO: Remove pub specifier
    pub fn output_to_assembly(&mut self, tokens: &Vec<Instruction>) {
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
