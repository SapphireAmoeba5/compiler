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
            match instr.op {
                Operation::Push => {
                    let asm = asm_push(instr);
                    assembly.push_str(&asm);
                }
                Operation::Dump => {
                    let asm = asm_dump(instr);
                    assembly.push_str(&asm);
                }
                _ => {}
            }
        }

        // Push code to return from program
        assembly.push_str(
            "    mov rax, 60
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
