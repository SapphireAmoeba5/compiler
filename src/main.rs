mod compiler;
mod interpreter;
mod logger;
mod operation;

use clap::Parser;

use compiler::*;
use interpreter::*;

// TODO: Remove this after debugging
use operation::*;

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
}
