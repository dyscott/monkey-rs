#[macro_use]
mod token;
mod lexer;
mod repl;

use crate::repl::start;
use whoami::username;

fn main() {
    let user = username();
    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");
    start(&mut std::io::stdin(), &mut std::io::stdout()).unwrap();
}
