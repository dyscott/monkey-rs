mod environment;
mod object;
mod builtins;

use std::{cell::RefCell, rc::Rc};

use crate::{
    parser::ast::{Expression, Node, Program, Statement},
    lexer::token::Token,
};
use crate::token;
use anyhow::{anyhow, Result};
use environment::Environment;
use object::Object;

#[cfg(test)]
mod tests;

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Default for Evaluator {
    // Create a new evaluator with a default environment
    fn default() -> Self {
        Self::new(Rc::new(RefCell::new(Environment::default())))
    }
}

impl Evaluator {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Evaluator { env }
    }

    // Entry-point to evaluate a program
    pub fn eval(&mut self, program: &Program) -> Result<Object> {
        self.eval_node(Node::Program(program))
    }

    // Evaluate a AST node
    fn eval_node(&mut self, node: Node) -> Result<Object> {
        stacker::maybe_grow(32 * 1024, 1024 * 1024, || {
            match node {
                Node::Program(program) => self.eval_program(&program),
                Node::Statement(statement) => self.eval_statement(statement),
                Node::Expression(expression) => self.eval_expression(expression),
            }
        })
    }

    // Evaluate a program node
    fn eval_program(&mut self, program: &Program) -> Result<Object> {
        let mut result = Ok(Object::Null);

        for statement in &program.statements {
            result = self.eval_node(Node::Statement(&statement));

            match result {
                Ok(Object::ReturnValue(value)) => return Ok(*value),
                Ok(_) => {}
                Err(_) => return result,
            }
        }

        result
    }

    // Evaluate a block statement
    fn eval_block_statement(&mut self, statements: &Vec<Statement>) -> Result<Object> {
        let mut result = Ok(Object::Null);

        for statement in statements {
            result = self.eval_node(Node::Statement(&statement));

            match result {
                Ok(Object::ReturnValue(_)) | Err(_) => return result,
                Ok(_) => {}
            }
        }

        result
    }

    // Evaluate a statement node
    fn eval_statement(&mut self, statement: &Statement) -> Result<Object> {
        match statement {
            Statement::Expression(expression) => self.eval_node(Node::Expression(expression)),
            Statement::Block(block) => self.eval_block_statement(&block),
            Statement::Return(expression) => Ok(Object::ReturnValue(Box::new(
                self.eval_node(Node::Expression(expression))?,
            ))),
            Statement::Let(name, expression) => self.eval_let_statement(name, expression),
        }
    }

    // Evaluate a let statement
    fn eval_let_statement(&mut self, name: &String, expression: &Expression) -> Result<Object> {
        let value = self.eval_node(Node::Expression(expression))?;
        self.env.borrow_mut().set(name, value.clone());
        Ok(Object::Null)
    }

    // Evaluate an expression node
    fn eval_expression(&mut self, expression: &Expression) -> Result<Object> {
        match expression {
            Expression::Integer(value) => Ok(Object::Integer(value.clone())),
            Expression::Boolean(value) => Ok(Object::Boolean(value.clone())),
            Expression::String(value) => Ok(Object::String(value.clone())),
            Expression::Array(value) => self.eval_array_literal_expression(value),
            Expression::Prefix(op, right) => self.eval_prefix_expression(op, right),
            Expression::Infix(op, left, right) => self.eval_infix_expression(op, left, right),
            Expression::If(condition, consequence, alternative) => {
                self.eval_if_expression(condition, consequence, alternative)
            }
            Expression::Identifier(name) => self.eval_identifier_expression(name),
            Expression::Function(params, body) => Ok(Object::Function(
                params.clone(),
                body.clone(),
                self.env.clone(),
            )),
            Expression::Call(function, args) => self.eval_function_call_expression(function, args),
            Expression::Index(left, index) => self.eval_index_expression(left, index),
        }
    }

    // Evaluate an array literal expression
    fn eval_array_literal_expression(&mut self, elements: &Vec<Expression>) -> Result<Object> {
        let elements = elements
            .iter()
            .map(|e| self.eval_node(Node::Expression(e)))
            .collect::<Result<Vec<Object>>>()?;
        Ok(Object::Array(elements))
    }

    // Evaluate a prefix expression
    fn eval_prefix_expression(&mut self, op: &Token, right: &Expression) -> Result<Object> {
        let right = self.eval_node(Node::Expression(right))?;
        match op {
            token!(!) => Ok(Object::Boolean(!right.is_truthy())),
            token!(-) => match right {
                Object::Integer(value) => Ok(Object::Integer(-value)),
                _ => Err(anyhow!("unknown operator: {}{}", op, right.type_name())),
            },
            _ => Err(anyhow!("unknown operator: {}{}", op, right.type_name())),
        }
    }

    // Evaluate an infix expression
    fn eval_infix_expression(
        &mut self,
        op: &Token,
        left: &Expression,
        right: &Expression,
    ) -> Result<Object> {
        let left = self.eval_node(Node::Expression(left))?;
        let right = self.eval_node(Node::Expression(right))?;

        match (op, &left, &right) {
            (op, Object::Integer(left), Object::Integer(right)) => {
                self.eval_integer_infix_expression(op, *left, *right)
            }
            (op, Object::String(left), Object::String(right)) => {
                self.eval_string_infix_expression(op, left, right)
            }
            (token!(==), Object::Boolean(left), Object::Boolean(right)) => {
                Ok(Object::Boolean(left == right))
            }
            (token!(!=), Object::Boolean(left), Object::Boolean(right)) => {
                Ok(Object::Boolean(left != right))
            }
            (_, left, right) if left.type_name() != right.type_name() => Err(anyhow!(
                "type mismatch: {} {} {}",
                left.type_name(),
                op,
                right.type_name()
            )),
            _ => Err(anyhow!(
                "unknown operator: {} {} {}",
                left.type_name(),
                op,
                right.type_name()
            )),
        }
    }

    // Evaluate an integer infix expression
    fn eval_integer_infix_expression(
        &mut self,
        op: &Token,
        left: i64,
        right: i64,
    ) -> Result<Object> {
        match op {
            token!(+) => Ok(Object::Integer(left + right)),
            token!(-) => Ok(Object::Integer(left - right)),
            token!(/) => Ok(Object::Integer(left / right)),
            token!(*) => Ok(Object::Integer(left * right)),
            token!(<) => Ok(Object::Boolean(left < right)),
            token!(>) => Ok(Object::Boolean(left > right)),
            token!(==) => Ok(Object::Boolean(left == right)),
            token!(!=) => Ok(Object::Boolean(left != right)),
            _ => Err(anyhow!("unknown operator: INTEGER {} INTEGER", op)),
        }
    }

    // Evaluate a string infix expression
    fn eval_string_infix_expression(
        &mut self,
        op: &Token,
        left: &String,
        right: &String,
    ) -> Result<Object> {
        match op {
            token!(+) => Ok(Object::String(left.to_owned() + right)),
            token!(==) => Ok(Object::Boolean(left == right)),
            token!(!=) => Ok(Object::Boolean(left != right)),
            _ => Err(anyhow!("unknown operator: STRING {} STRING", op)),
        }
    }

    // Evaluate an if expression
    fn eval_if_expression(
        &mut self,
        condition: &Expression,
        consequence: &Statement,
        alternative: &Option<Box<Statement>>,
    ) -> Result<Object> {
        let condition = self.eval_node(Node::Expression(condition))?;

        if condition.is_truthy() {
            self.eval_node(Node::Statement(consequence))
        } else {
            match alternative {
                Some(alternative) => self.eval_node(Node::Statement(alternative)),
                None => Ok(Object::Null),
            }
        }
    }

    // Evaluate an identifier expression
    fn eval_identifier_expression(&mut self, name: &String) -> Result<Object> {
        match self.env.borrow().get(name) {
            Some(value) => Ok(value.clone()),
            None => match builtins::get_builtin(name) {
                Some(builtin) => Ok(Object::BuiltInFunction(builtin)),
                None => Err(anyhow!("identifier not found: {}", name)),
            },
        }
    }

    // Evaluate a function call expression
    fn eval_function_call_expression(
        &mut self,
        function: &Expression,
        args: &Vec<Expression>,
    ) -> Result<Object> {
        let function = self.eval_node(Node::Expression(function))?;
        let args = args
            .iter()
            .map(|a| self.eval_node(Node::Expression(a)))
            .collect::<Result<Vec<Object>>>()?;

        // Get the function's parameters, body, and environment
        let (params, body, env) = match function {
            Object::Function(params, body, env) => (params, body, env),
            // Built-in functions are called directly
            Object::BuiltInFunction(builtin) => return builtin(args),
            _ => return Err(anyhow!("not a function: {}", function.type_name())),
        };

        // Extend the environment with the function's arguments
        let mut env = Environment::new(Some(env));
        for (param, arg) in params.iter().zip(args) {
            env.set(param, arg);
        }

        // Evaluate the function's body in the extended environment
        let mut evaluator = Evaluator::new(Rc::new(RefCell::new(env)));
        let evaluated = evaluator.eval_node(Node::Statement(&body));

        // Unwrap the return value if it exists
        match evaluated {
            Ok(Object::ReturnValue(value)) => Ok(*value),
            _ => evaluated,
        }
    }

    // Evaluate an index expression
    fn eval_index_expression(&mut self, left: &Expression, index: &Expression) -> Result<Object> {
        let left = self.eval_node(Node::Expression(left))?;
        let index = self.eval_node(Node::Expression(index))?;

        match (&left, &index) {
            (Object::Array(elements), Object::Integer(index)) => {
                if *index < 0 || *index >= elements.len() as i64 {
                    return Ok(Object::Null);
                }
                Ok(elements[*index as usize].clone())
            }
            (Object::String(string), Object::Integer(index)) => {
                match string.chars().nth(*index as usize) {
                    Some(char) => Ok(Object::String(char.to_string())),
                    None => Ok(Object::Null),
                }
            }
            _ => Err(anyhow!(
                "index operator not supported: {}[{}]",
                left.type_name(),
                index.type_name()
            )),
        }
    }
}
