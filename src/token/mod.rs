use std::fmt::{self, Display, Formatter};

use crate::token;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(String),

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

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

impl Display for Token {
    // Format the token for printing
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            token!(ILLEGAL) => write!(f, "ILLEGAL"),
            token!(EOF) => write!(f, "EOF"),
            Token::Ident(value) => write!(f, "IDENT({})", value),
            Token::Int(value) => write!(f, "INT({})", value),
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
            token!('{') => write!(f, "{{"),
            token!('}') => write!(f, "}}"),
            token!('(') => write!(f, "("),
            token!(')') => write!(f, ")"),
            token!(FUNCTION) => write!(f, "FUNCTION"),
            token!(LET) => write!(f, "LET"),
            token!(TRUE) => write!(f, "TRUE"),
            token!(FALSE) => write!(f, "FALSE"),
            token!(IF) => write!(f, "IF"),
            token!(ELSE) => write!(f, "ELSE"),
            token!(RETURN) => write!(f, "RETURN"),
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
    (ILLEGAL) => { Token::Illegal };
    (EOF) => { Token::Eof };
    (IDENT($value:expr)) => { Token::Ident($value.to_string()) };
    (INT($value:expr)) => { Token::Int($value.to_string()) };
    (=) => { Token::Assign };
    (+) => { Token::Plus };
    (-) => { Token::Minus };
    (!) => { Token::Bang };
    (*) => { Token::Asterisk };
    (/) => { Token::Slash };
    (==) => { Token::Eq };
    (!=) => { Token::NotEq };
    (<) => { Token::Lt };
    (>) => { Token::Gt };
    (,) => { Token::Comma };
    (;) => { Token::Semicolon };
    ('{') => { Token::LBrace };
    ('}') => { Token::RBrace };
    ('(') => { Token::LParen };
    (')') => { Token::RParen };
    (FUNCTION) => { Token::Function };
    (LET) => { Token::Let };
    (TRUE) => { Token::True };
    (FALSE) => { Token::False };
    (IF) => { Token::If };
    (ELSE) => { Token::Else };
    (RETURN) => { Token::Return };
}