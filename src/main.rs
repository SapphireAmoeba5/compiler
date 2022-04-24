mod compiler;
mod interpreter;
mod logger;
mod operation;

use clap::Parser;
use compiler::*;
use interpreter::*;

#[derive(Parser)]
struct Arguments {
    #[clap()]
    input_file: String,
    #[clap(short = 'o', long = "output-file", default_value = "output.asm")]
    output_file: String,
    #[clap(short = 'i', long = "interpret")]
    interpret: bool,
}

fn main() {
    let args = Arguments::parse();

    if !args.interpret {
        let s = Compiler::new(&args.input_file, &args.output_file);
        match s {
            Ok(_) => {}
            Err(_) => {
                error_println!("Compilation failed");
            }
        }
    } else {
        let mut s = Interpreter::new(&args.input_file);
        let result = s.interpret();
    }
}
