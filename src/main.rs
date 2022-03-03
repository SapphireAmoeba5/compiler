mod compiler;
mod interpreter;
mod logger;
mod operation;

use clap::Parser;

use compiler::*;

#[derive(Parser)]
struct Arguments {
    #[clap()]
    input_file: String,
    #[clap(short = 'o', long = "output-file", default_value = "output.asm")]
    output_file: String,
}

fn main() {
    let args = Arguments::parse();

    let s = Compiler::new(&args.input_file, &args.output_file);

    match s {
        Ok(_) => {}
        Err(_) => {
            error_println!("Compilation failed");
        }
    }
}
