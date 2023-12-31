use crate::compiler::Compiler;
use crate::eval::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::VM;

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

pub fn benchmark_file(path: String) -> Result<()> {
    // Open the file
    let path = Path::new(&path);
    let mut file = File::open(path)?;

    // Read the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the file
    let start = Instant::now(); // Start timer
    let lexer = Lexer::new(contents.clone());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let duration = start.elapsed(); // Stop timer

    if !parser.errors.is_empty() {
        println!("Error(s) occurred during parsing:");
        for error in parser.errors {
            println!("\tError: {}", error);
        }
        return Ok(());
    }

    println!("Parsing took: {:?}", duration);

    // Evaluate the file
    let mut evaluator = Evaluator::default();

    let start = Instant::now(); // Start timer
    let evaluated = evaluator.eval(&program);
    let duration = start.elapsed(); // Stop timer
    if let Err(error) = evaluated {
        println!("Error occurred during evaluation:");
        println!("\tError: {}", error)
    }
    println!("Evaluation took: {:?}", duration);

    // Compile the file
    let mut compiler = Compiler::new();

    let start = Instant::now(); // Start timer
    let result = compiler.compile(&program);
    let duration = start.elapsed(); // Stop timer

    if let Err(error) = result {
        println!("Error occurred during compilation:");
        println!("\tError: {}", error);
        return Ok(());
    }

    println!("Compilation took: {:?}", duration);

    let bytecode = compiler.bytecode();

    // Run the file
    let mut vm = VM::new(bytecode);

    let start = Instant::now(); // Start timer
    let result = vm.run();
    let duration = start.elapsed(); // Stop timer

    if let Err(error) = result {
        println!("Error occurred during execution:");
        println!("\tError: {}", error);
        return Ok(());
    }

    println!("Execution took: {:?}", duration);

    Ok(())
}