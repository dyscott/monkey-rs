#![allow(dead_code)]

use clap::Parser;

#[macro_use]
mod lexer;
mod repl;
mod parser;
mod object;
mod eval;
mod run;
mod code;
mod compiler;
mod vm;

use crate::repl::start;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to run
    #[arg(short, long)]
    file: Option<String>,

    /// Whether to run in compiled mode
    #[arg(short, long, default_value = "false")]
    compiled: bool,
}

fn main() {
    // Check for file argument
    let args = Args::parse();
    if let Some(file) = args.file {
        run::run_file(file, args.compiled).unwrap();
        return;
    }

    start(&mut std::io::stdin(), args.compiled).unwrap();
}
