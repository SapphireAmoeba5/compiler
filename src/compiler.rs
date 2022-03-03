mod generator;
mod lexer;

use std::fs::File;
use std::io::Write;
use std::time::Instant;

use crate::operation::*;
use crate::{debug_println, error_println, info_println, info_println_if};
use generator::*;
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

        s.output_to_assembly(&lexed)?;

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
    fn output_to_assembly(&mut self, tokens: &Vec<Instruction>) -> Result<(), ()> {
        // Output generated assembly to file
        let mut out_file = match File::create(&self.output_file) {
            Ok(s) => s,
            Err(e) => {
                error_println!("Error opening output file :: Error: {}", e);
                return Err(());
            }
        };

        let mut generater = AsmGenerator::new();
        let assembly = generater.compile_tokens(&tokens)?;

        match out_file.write(assembly.as_bytes()) {
            Ok(_) => {}
            _ => {
                error_println!("Unknown error writing to output file");
            }
        }

        Ok(())
    }
}
