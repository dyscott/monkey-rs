use crate::code::{Instructions, Opcode, make};
use crate::token;
use crate::lexer::token::Token;
use crate::object::Object;
use crate::parser::ast::{Expression, Node, Program, Statement};
use anyhow::Result;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

impl Compiler {
    // Create a new compiler
    pub fn new() -> Self {
        Self {
            instructions: Instructions::new(),
            constants: vec![],
        }
    }

    // Compile a full program
    pub fn compile(&mut self, program: &Program) -> Result<()> {
        self.compile_node(&Node::Program(program))
    }
    // Compile from an AST node
    pub fn compile_node(&mut self, node: &Node) -> Result<()> {
        match node {
            Node::Program(program) => self.compile_program(program)?,
            Node::Statement(statement) => self.compile_statement(statement)?,
            Node::Expression(expression) => self.compile_expression(expression)?,
        };
        Ok(())
    }

    // Compile a program AST node
    pub fn compile_program(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            self.compile_node(&Node::Statement(&statement))?;
        }
        Ok(())
    }

    // Compile a statement AST node
    pub fn compile_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expression) => {
                self.compile_node(&Node::Expression(expression))?;
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    // Compile an expression AST node
    pub fn compile_expression(&mut self, expression: &Expression) -> Result<()> {
        match expression {
            Expression::Infix(op, left, right) => {
                self.compile_node(&Node::Expression(left))?;
                self.compile_node(&Node::Expression(right))?;
                match op {
                    token!(+) => self.emit(Opcode::OpAdd, vec![]),
                    _ => unimplemented!(),
                };

            }
            Expression::Integer(value) => {
                let integer = Object::Integer(*value);
                let constant = self.add_constant(integer);
                self.emit(Opcode::OpConstant, vec![constant as u64]);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    // Add a constant to the compiler
    pub fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);
        return self.constants.len() - 1
    }

    // Emit an instruction
    pub fn emit(&mut self, opcode: Opcode, operands: Vec<u64>) -> usize {
        let instruction = make(opcode, operands);
        let position = self.add_instruction(instruction);
        return position;
    }

    // Add an instruction to the compiler
    pub fn add_instruction(&mut self, instruction: Instructions) -> usize {
        let position = self.instructions.len();
        self.instructions.extend(instruction);
        return position;
    }

    // Get the compiled bytecode
    pub fn bytecode(self) -> Bytecode {
        Bytecode {
            instructions: self.instructions,
            constants: self.constants,
        }
    }
}
