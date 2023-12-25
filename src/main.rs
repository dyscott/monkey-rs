#![allow(dead_code)]

#[macro_use]
mod lexer;
mod repl;
mod parser;
mod object;
mod eval;
mod run;
mod code;
mod compiler;

use crate::repl::start;
use std::env::args;

fn main() {
    // Check for file argument
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        run::run_file(args[1].clone()).unwrap();
        return;
    }

    start(&mut std::io::stdin()).unwrap();
}
