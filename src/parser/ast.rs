use std::fmt::{self, Display, Formatter};

use crate::token::Token;

pub enum Node {
    Program(Program),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
}

impl Display for Statement {
    // Pretty print parsed statements
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Statement::Let(name, value) => {
                write!(f, "let {name} = {value};")
            }
            Statement::Return(value) => {
                write!(f, "return {value};")
            }
            Statement::Expression(value) => {
                write!(f, "{}", value)
            }
            Statement::Block(statements) => {
                let output = statements
                    .iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<String>>()
                    .join("");
                write!(f, "{}", output)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Boolean(bool),
    Prefix(Token, Box<Expression>),
    Infix(Token, Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    Function(Vec<String>, Box<Statement>),
    Call(Box<Expression>, Vec<Expression>),
}
impl Display for Expression {
    // Pretty print parsed expressions
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(value) => {
                write!(f, "{}", value)
            }
            Expression::Integer(value) => {
                write!(f, "{}", value)
            }
            Expression::Boolean(value) => {
                write!(f, "{}", value)
            }
            Expression::Prefix(op, right) => {
                write!(f, "({}{})", op, right)
            }
            Expression::Infix(op, left, right) => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expression::If(condition, consequence, Some(alternative)) => {
                write!(f, "if ({}) {{{}}} else {{{}}}", condition, consequence, alternative)
            }
            Expression::If(condition, consequence, None) => {
                write!(f, "if ({}) {{{}}}", condition, consequence)
            }
            Expression::Function(params, body) => {
                let params = params
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "fn({}) {{{}}}", params, body)
            }
            Expression::Call(function, args) => {
                let args = args
                    .iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({})", function, args)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    // Pretty print parsed programs
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let output = self
            .statements
            .iter()
            .map(|s| format!("{}", s))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", output)
    }
}
