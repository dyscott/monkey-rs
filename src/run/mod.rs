use crate::eval::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run_file(path: String) -> Result<()> {
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

    // Evaluate the file
    let mut evaluator = Evaluator::default();
    let evaluated = evaluator.eval(&program);
    match evaluated {
        Ok(_) => {
            // Ignore the result
        }
        Err(error) => {
            println!("Error occurred during evaluation:");
            println!("\tError: {}", error)
        }
    }

    Ok(())
}
