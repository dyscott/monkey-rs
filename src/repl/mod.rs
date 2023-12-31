use crate::eval::Evaluator;
use crate::parser::Parser;
use crate::vm::VM;
use crate::{compiler::Compiler, lexer::Lexer};

use anyhow::Result;
use std::io::{stdout, Stdin, Write};
use whoami::username;

const PROMPT: &str = ">> ";

pub fn start(input: &mut Stdin, eval: bool) -> Result<()> {
    let mut buffer = String::new();

    let user = username();
    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands or 'exit' to exit the REPL");

    let mut evaluator = Evaluator::default();
    let mut compiler = Compiler::new();
    let mut vm = VM::default();

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

        if eval {
            // Use the evaluator instead of the compiler
            let evaluated = evaluator.eval(&program);
            match evaluated {
                Ok(evaluated) => {
                    println!("{}", evaluated);
                }
                Err(error) => {
                    println!("Error occurred during evaluation:");
                    println!("\tError: {}", error)
                }
            }
            continue;
        }

        // Compile the input
        let result = compiler.compile(&program);
        if let Err(error) = result {
            println!("Error occurred during compilation:");
            println!("\tError: {}", error);
            compiler.reset();
            continue;
        }
        let bytecode = compiler.bytecode().clone();
        compiler.reset();

        // Run the input
        vm.reset(bytecode);
        let result = vm.run();
        if let Err(error) = result {
            println!("Error occurred during execution:");
            println!("\tError: {}", error);
            continue;
        }

        // Print the stack top
        let stack_elem = vm.last_popped_stack_elem();
        println!("{}", stack_elem);
    }

    Ok(())
}
