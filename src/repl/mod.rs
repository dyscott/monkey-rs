use crate::lexer::Lexer;
use crate::token::Token;

use std::io::{Error, Stdin, Stdout, Write};

const PROMPT: &str = ">> ";

pub fn start(input: &mut Stdin, output: &mut Stdout) -> Result<(), Error> {
    let mut buffer = String::new();

    loop {
        buffer.clear();
        output.write_all(PROMPT.as_bytes())?;
        output.flush()?;
        input.read_line(&mut buffer)?;

        let mut lexer = Lexer::new(buffer.clone());
        let mut token = lexer.next_token();
        while token != token!(EOF) {
            output.write_all(format!("{}\n", token).as_bytes())?;
            token = lexer.next_token();
        }
        output.flush()?;
    }
}