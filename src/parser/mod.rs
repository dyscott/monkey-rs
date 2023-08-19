pub mod ast;

#[cfg(test)]
mod tests;

use crate::lexer::Lexer;
use crate::lexer::token::Token;
use crate::token;
use anyhow::{anyhow, Result};
use ast::*;

#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

impl From<&Token> for Precedence {
    fn from(token: &Token) -> Self {
        match token {
            token!(==) | token!(!=) => Precedence::Equals,
            token!(<) | token!(>) => Precedence::LessGreater,
            token!(+) | token!(-) => Precedence::Sum,
            token!(*) | token!(/) => Precedence::Product,
            token!('(') => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::new(),
        };

        // Read two tokens to set cur_token and peek_token
        parser.next_token();
        parser.next_token();

        parser
    }

    // Advance the parser by one token
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // Parse an entire program
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token != Token::Eof {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(err) => self.errors.push(err.to_string()),
            };
            self.next_token();
        }

        return program;
    }

    // Parse a statement
    fn parse_statement(&mut self) -> Result<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    // Parse a let statement
    fn parse_let_statement(&mut self) -> Result<Statement> {
        // Parse the name of the variable
        let name = match self.peek_token {
            Token::Ident(ref name) => name.clone(),
            _ => {
                return Err(anyhow!(
                    "Expected next token to be IDENT, got {:?} instead",
                    self.peek_token
                ))
            }
        };
        self.next_token();

        // Parse the assignment operator
        if self.peek_token != Token::Assign {
            return Err(anyhow!(
                "Expected next token to be =, got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();
        self.next_token();

        // Parse the expression
        let value = self.parse_expression(Precedence::Lowest)?;

        // Semi-colon is optional
        if self.peek_token == token!(;) {
            self.next_token();
        }

        Ok(Statement::Let(name, value))
    }

    // Parse a return statement
    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.next_token();

        // Parse the expression
        let value = self.parse_expression(Precedence::Lowest)?;

        // Semi-colon is optional
        if self.peek_token == token!(;) {
            self.next_token();
        }

        Ok(Statement::Return(value))
    }

    // Parse an expression statement
    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        // Semi-colon is optional
        if self.peek_token == token!(;) {
            self.next_token();
        }

        Ok(Statement::Expression(expression))
    }

    // Parse an expression
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
        // Parse the initial prefix
        let prefix = self.parse_prefix()?;

        let mut left = prefix;
        while self.peek_token != token!(;) && precedence < Precedence::from(&self.peek_token) {
            // Parse the infix
            let infix = match self.parse_infix(&left) {
                Ok(infix) => infix,
                Err(err) => return Err(err),
            };

            // Update the left side of the expression
            left = infix;
        }

        Ok(left)
    }

    // Parse a prefix
    fn parse_prefix(&mut self) -> Result<Expression> {
        match self.cur_token {
            Token::Ident(ref value) => self.parse_identifier(value.clone()),
            Token::Int(ref value) => self.parse_integer(value.clone()),
            Token::String(ref value) => self.parse_string(value.clone()),
            token!(TRUE) => self.parse_boolean(true),
            token!(FALSE) => self.parse_boolean(false),
            token!(!) | token!(-) => self.parse_prefix_expression(),
            token!('(') => self.parse_group(),
            token!(IF) => self.parse_if(),
            token!(FUNCTION) => self.parse_function(),
            _ => Err(anyhow!(
                "No prefix parse function for {} found",
                self.cur_token
            )),
        }
    }

    // Parse an identifier
    fn parse_identifier(&mut self, value: String) -> Result<Expression> {
        Ok(Expression::Identifier(value))
    }

    // Parse an integer
    fn parse_integer(&mut self, value: String) -> Result<Expression> {
        // Parse the integer
        let int = value.parse::<i64>()?;
        Ok(Expression::Integer(int))
    }

    // Parse a boolean
    fn parse_boolean(&mut self, value: bool) -> Result<Expression> {
        Ok(Expression::Boolean(value))
    }

    // Parse a string
    fn parse_string(&mut self, value: String) -> Result<Expression> {
        Ok(Expression::String(value))
    }

    // Parse a prefix expression
    fn parse_prefix_expression(&mut self) -> Result<Expression> {
        // Save the operator
        let op = self.cur_token.clone();
        self.next_token();
        // Parse the right side of the expression
        let right = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression::Prefix(op, Box::new(right)))
    }

    // Parse a group expression
    fn parse_group(&mut self) -> Result<Expression> {
        // Parse the expression inside the parentheses
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        // Parse the closing parenthesis
        if self.peek_token != token!(')') {
            return Err(anyhow!(
                "Expected next token to be ), got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();
        Ok(expression)
    }

    // Parse an if expression
    fn parse_if(&mut self) -> Result<Expression> {
        // Parse the if condition
        if self.peek_token != token!('(') {
            return Err(anyhow!(
                "Expected next token to be (, got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token != token!(')') {
            return Err(anyhow!(
                "Expected next token to be ), got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();

        // Parse the if body
        if self.peek_token != token!('{') {
            return Err(anyhow!(
                "Expected next token to be {{, got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();
        let consequence = self.parse_block()?;

        // Parse the else body
        let alternative = if self.peek_token == token!(ELSE) {
            self.next_token();
            if self.peek_token != token!('{') {
                return Err(anyhow!(
                    "Expected next token to be {{, got {:?} instead",
                    self.peek_token
                ));
            }
            self.next_token();
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        return Ok(Expression::If(
            Box::new(condition),
            Box::new(consequence),
            alternative,
        ));
    }

    // Parse a function expression
    fn parse_function(&mut self) -> Result<Expression> {
        // Parse the function parameters
        if self.peek_token != token!('(') {
            return Err(anyhow!(
                "Expected next token to be (, got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();
        let parameters = self.parse_function_parameters()?;

        // Parse the function body
        if self.peek_token != token!('{') {
            return Err(anyhow!(
                "Expected next token to be {{, got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();
        let body = self.parse_block()?;

        return Ok(Expression::Function(parameters, Box::new(body)));
    }

    // Parse the parameters of a function
    fn parse_function_parameters(&mut self) -> Result<Vec<String>> {
        let mut parameters = Vec::new();

        // Zero parameters
        if self.peek_token == token!(')') {
            self.next_token();
            return Ok(parameters);
        }

        self.next_token();

        // Parse the first parameter
        match self.cur_token {
            Token::Ident(ref value) => parameters.push(value.clone()),
            _ => {
                return Err(anyhow!(
                    "Expected next token to be IDENT, got {:?} instead",
                    self.cur_token
                ))
            }
        };

        // Parse the remaining parameters
        while self.peek_token == token!(,) {
            self.next_token();
            self.next_token();
            match self.cur_token {
                Token::Ident(ref value) => parameters.push(value.clone()),
                _ => {
                    return Err(anyhow!(
                        "Expected next token to be IDENT, got {:?} instead",
                        self.cur_token
                    ))
                }
            };
        }

        // Parse the closing parenthesis
        if self.peek_token != token!(')') {
            return Err(anyhow!(
                "Expected next token to be ), got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();

        Ok(parameters)
    }

    // Parse a block statement
    fn parse_block(&mut self) -> Result<Statement> {
        let mut statements = Vec::new();

        self.next_token();

        // Parse all statements until the closing brace
        while self.cur_token != token!('}') {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => self.errors.push(err.to_string()),
            };
            self.next_token();
        }

        Ok(Statement::Block(statements))
    }

    // Parse an infix
    fn parse_infix(&mut self, left: &Expression) -> Result<Expression> {
        match self.peek_token {
            token!(+)
            | token!(-)
            | token!(/)
            | token!(*)
            | token!(==)
            | token!(!=)
            | token!(<)
            | token!(>) => self.parse_infix_expression(left.clone()),
            token!('(') => self.parse_call(left.clone()),
            _ => Err(anyhow!(
                "No infix parse function for {} found",
                self.peek_token
            )),
        }
    }

    // Parse an infix expression
    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression> {
        self.next_token();
        // Save the operator and its precedence
        let op = self.cur_token.clone();
        let precedence = Precedence::from(&op);
        self.next_token();
        // Parse the right side of the expression
        let right = self.parse_expression(precedence)?;
        Ok(Expression::Infix(
            op,
            Box::new(left.clone()),
            Box::new(right),
        ))
    }

    // Parse a call expression
    fn parse_call(&mut self, function: Expression) -> Result<Expression> {
        // Parse the function arguments
        self.next_token();
        let arguments = self.parse_call_arguments()?;

        Ok(Expression::Call(Box::new(function), arguments))
    }

    // Parse the arguments of a call expression
    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>> {
        let mut arguments = Vec::new();

        // Zero arguments
        if self.peek_token == token!(')') {
            self.next_token();
            return Ok(arguments);
        }

        self.next_token();

        // Parse the first argument
        let argument = self.parse_expression(Precedence::Lowest)?;
        arguments.push(argument);

        // Parse the remaining arguments
        while self.peek_token == token!(,) {
            self.next_token();
            self.next_token();
            let argument = self.parse_expression(Precedence::Lowest)?;
            arguments.push(argument);
        }

        // Parse the closing parenthesis
        if self.peek_token != token!(')') {
            return Err(anyhow!(
                "Expected next token to be ), got {:?} instead",
                self.peek_token
            ));
        }
        self.next_token();

        Ok(arguments)
    }
}
