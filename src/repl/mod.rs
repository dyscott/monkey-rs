use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::eval::Evaluator;

use anyhow::Result;
use std::io::{Stdin, stdout, Write};
use whoami::username;

const PROMPT: &str = ">> ";

pub fn start(input: &mut Stdin) -> Result<()> {
    let mut buffer = String::new();

    let user = username();
    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands or 'exit' to exit the REPL");

    let mut evaluator = Evaluator::default();
    loop {
        // Prompt and read input
        buffer.clear();
        print!("{}", PROMPT);
        stdout().flush()?;
        input.read_line(&mut buffer)?;

        // Check for exit
        if buffer.trim() == "exit" {
            break;
        }

        // Parse the input
        let lexer = Lexer::new(buffer.clone());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if !parser.errors.is_empty() {
            println!("Error(s) occurred during parsing:");
            for error in parser.errors {
                println!("\tError: {}", error);
            }
            continue;
        }

        // Evaluate the input
        let evaluated = evaluator.eval(&program);
        match evaluated {
            Ok(evaluated) => {
                println!("{}\n", evaluated);
            }
            Err(error) => {
                println!("Error occurred during evaluation:");
                println!("\tError: {}", error)
            }
        }
    }

    Ok(())
}
