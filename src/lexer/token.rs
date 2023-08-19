use std::fmt::{self, Display, Formatter};

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(String),
    String(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Eq,
    NotEq,
    Lt,
    Gt,

    // Delimiters
    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token {
    // Format the token for printing
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            token!(ILLEGAL) => write!(f, "ILLEGAL"),
            token!(EOF) => write!(f, "EOF"),
            Token::Ident(value) => write!(f, "{value}"),
            Token::Int(value) => write!(f, "{value}"),
            Token::String(value) => write!(f, "{value}"),
            token!(=) => write!(f, "="),
            token!(+) => write!(f, "+"),
            token!(-) => write!(f, "-"),
            token!(!) => write!(f, "!"),
            token!(*) => write!(f, "*"),
            token!(/) => write!(f, "/"),
            token!(==) => write!(f, "=="),
            token!(!=) => write!(f, "!="),
            token!(<) => write!(f, "<"),
            token!(>) => write!(f, ">"),
            token!(,) => write!(f, ","),
            token!(;) => write!(f, ";"),
            token!(:) => write!(f, ":"),
            token!('{') => write!(f, "{{"),
            token!('}') => write!(f, "}}"),
            token!('(') => write!(f, "("),
            token!(')') => write!(f, ")"),
            token!('[') => write!(f, "["),
            token!(']') => write!(f, "]"),
            token!(FUNCTION) => write!(f, "fn"),
            token!(LET) => write!(f, "let"),
            token!(TRUE) => write!(f, "true"),
            token!(FALSE) => write!(f, "false"),
            token!(IF) => write!(f, "if"),
            token!(ELSE) => write!(f, "else"),
            token!(RETURN) => write!(f, "return"),
        }
    }
}

// Look up an identifier to see if it is a keyword
pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => token!(FUNCTION),
        "let" => token!(LET),
        "true" => token!(TRUE),
        "false" => token!(FALSE),
        "if" => token!(IF),
        "else" => token!(ELSE),
        "return" => token!(RETURN),
        _ => token!(IDENT(ident)),
    }
}

// Macro to make it easier to create tokens
#[macro_export]
macro_rules! token {
    (ILLEGAL) => {
        Token::Illegal
    };
    (EOF) => {
        Token::Eof
    };
    (IDENT($value:expr)) => {
        Token::Ident($value.to_string())
    };
    (INT($value:expr)) => {
        Token::Int($value.to_string())
    };
    (STRING($value:expr)) => {
        Token::String($value.to_string())
    };
    (=) => {
        Token::Assign
    };
    (+) => {
        Token::Plus
    };
    (-) => {
        Token::Minus
    };
    (!) => {
        Token::Bang
    };
    (*) => {
        Token::Asterisk
    };
    (/) => {
        Token::Slash
    };
    (==) => {
        Token::Eq
    };
    (!=) => {
        Token::NotEq
    };
    (<) => {
        Token::Lt
    };
    (>) => {
        Token::Gt
    };
    (,) => {
        Token::Comma
    };
    (;) => {
        Token::Semicolon
    };
    (:) => {
        Token::Colon
    };
    ('{') => {
        Token::LBrace
    };
    ('}') => {
        Token::RBrace
    };
    ('(') => {
        Token::LParen
    };
    (')') => {
        Token::RParen
    };
    ('[') => {
        Token::LBracket
    };
    (']') => {
        Token::RBracket
    };
    (FUNCTION) => {
        Token::Function
    };
    (LET) => {
        Token::Let
    };
    (TRUE) => {
        Token::True
    };
    (FALSE) => {
        Token::False
    };
    (IF) => {
        Token::If
    };
    (ELSE) => {
        Token::Else
    };
    (RETURN) => {
        Token::Return
    };
}
