use crate::compiler::Compiler;
use crate::eval::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::VM;

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run_file(path: String, compiled: bool) -> Result<()> {
    // Open the file
    let path = Path::new(&path);
    let mut file = File::open(path)?;

    // Read the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the file
    let lexer = Lexer::new(contents);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        println!("Error(s) occurred during parsing:");
        for error in parser.errors {
            println!("\tError: {}", error);
        }
        return Ok(());
    }

    if compiled {
        // Compile the file
        let mut compiler = Compiler::new();
        let result = compiler.compile(&program);
        if let Err(error) = result {
            println!("Error occurred during compilation:");
            println!("\tError: {}", error);
            return Ok(());
        }
        let bytecode = compiler.bytecode();

        // Run the file
        let mut vm = VM::new(bytecode);
        let result = vm.run();
        if let Err(error) = result {
            println!("Error occurred during execution:");
            println!("\tError: {}", error);
            return Ok(());
        }
        return Ok(());
    }

    // Fall back to interpreted mode
    let mut evaluator = Evaluator::default();
    let evaluated = evaluator.eval(&program);
    if let Err(error) = evaluated {
        println!("Error occurred during evaluation:");
        println!("\tError: {}", error)
    }

    Ok(())
}
