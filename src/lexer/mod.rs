use crate::{token, token::Token};

#[cfg(test)]
mod tests;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        return lexer;
    }

    // Read the next character and advance the position
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = match self.input.chars().nth(self.read_position) {
                Some(ch) => ch,
                None => '\0',
            };
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Read the next character without advancing the position
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return match self.input.chars().nth(self.read_position) {
                Some(ch) => ch,
                None => '\0',
            };
        }
    }

    // Read and return the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            // Read multi-character tokens
            '=' if self.peek_char() == '=' => {
                self.read_char();
                token!(==)
            }
            '!' if self.peek_char() == '=' => {
                self.read_char();
                token!(!=)
            }

            // Read single character tokens
            '=' => token!(=),
            '+' => token!(+),
            ',' => token!(,),
            ';' => token!(;),
            '-' => token!(-),
            '!' => token!(!),
            '*' => token!(*),
            '/' => token!(/),
            '<' => token!(<),
            '>' => token!(>),
            '(' => token!('('),
            ')' => token!(')'),
            '{' => token!('{'),
            '}' => token!('}'),
            '\0' => token!(EOF),

            // Read identifier
            c if is_letter(c) => return self.read_identifier(),

            // Read number
            c if c.is_ascii_digit() => return self.read_number(),
            _ => token!(ILLEGAL),
        };

        self.read_char();
        return token;
    }

    // Read and return an identifier
    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        let literal = &self.input[position..self.position];
        return token::lookup_ident(literal);
    }

    // Read and return a number
    fn read_number(&mut self) -> Token {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let literal = &self.input[position..self.position];
        return token!(INT(literal));
    }

    // Advance the lexer past any whitespace
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

// Check if a character is a letter (or underscore)
fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}