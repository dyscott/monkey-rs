use crate::lexer::Lexer;
use crate::parser::Parser;

use anyhow::Result;
use std::io::{Stdin, Stdout, Write};

const PROMPT: &str = ">> ";
const MONKEY_FACE: &str = r#"            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

pub fn start(input: &mut Stdin, output: &mut Stdout) -> Result<()> {
    let mut buffer = String::new();

    loop {
        buffer.clear();
        output.write_all(PROMPT.as_bytes())?;
        output.flush()?;
        input.read_line(&mut buffer)?;

        let lexer = Lexer::new(buffer.clone());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        if !parser.errors.is_empty() {
            print_parser_errors(output, parser.errors)?;
            continue;
        }

        output.write_all(format!("{:?}\n", program).as_bytes())?;
        output.flush()?;
    }
}

pub fn print_parser_errors(output: &mut Stdout, errors: Vec<String>) -> Result<()> {
    output.write_all(MONKEY_FACE.as_bytes())?;
    output.write_all(b"Whoops! We ran into some monkey business here!\n")?;
    output.write_all(b" parser errors:\n")?;
    for error in errors {
        output.write_all(format!("\t{}\n", error).as_bytes())?;
    }
    Ok(())
}
