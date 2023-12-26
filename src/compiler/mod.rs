pub mod symbol_table;

use crate::code::{make, Instructions, Opcode};
use crate::lexer::token::Token;
use crate::object::Object;
use crate::parser::ast::{Expression, Node, Program, Statement};
use crate::token;
use anyhow::{anyhow, Result};
use symbol_table::SymbolTable;

#[cfg(test)]
mod tests;

macro_rules! emit {
    ($self:ident, $opcode:expr) => {
        $self.emit($opcode, vec![])
    };
    ($self:ident, $opcode:expr, [$($operand:expr),*]) => {
        $self.emit($opcode, vec![$($operand),*])
    };
}

#[derive(Debug, PartialEq, Clone)]
pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
    symbol_table: SymbolTable,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

#[derive(Debug, PartialEq, Clone)]
struct EmittedInstruction {
    opcode: Opcode,
    position: usize,
}

impl Compiler {
    // Create a new compiler
    pub fn new() -> Self {
        Self {
            instructions: Instructions::new(),
            constants: vec![],
            last_instruction: None,
            previous_instruction: None,
            symbol_table: SymbolTable::new(),
        }
    }

    // Reset compiler for reuse (for REPL)
    pub fn reset(&mut self) {
        self.instructions = Instructions::new();
        self.last_instruction = None;
        self.previous_instruction = None;
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
                emit!(self, Opcode::OpPop);
            }
            Statement::Block(statements) => {
                for statement in statements {
                    self.compile_node(&Node::Statement(statement))?;
                }
            }
            Statement::Let(name, expression) => {
                self.compile_node(&Node::Expression(expression))?;
                let symbol = self.symbol_table.define(name);
                emit!(self, Opcode::OpSetGlobal, [symbol.index as u64]);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    // Compile an expression AST node
    pub fn compile_expression(&mut self, expression: &Expression) -> Result<()> {
        match expression {
            Expression::Integer(value) => {
                let integer = Object::Integer(*value);
                let constant = self.add_constant(integer);
                emit!(self, Opcode::OpConstant, [constant as u64]);
            }
            Expression::Boolean(value) => {
                match value {
                    true => emit!(self, Opcode::OpTrue),
                    false => emit!(self, Opcode::OpFalse),
                };
            }
            Expression::String(value) => {
                let string = Object::String(value.clone());
                let constant = self.add_constant(string);
                emit!(self, Opcode::OpConstant, [constant as u64]);
            }
            Expression::Hash(pairs) => {
                for (key, value) in pairs {
                    self.compile_node(&Node::Expression(key))?;
                    self.compile_node(&Node::Expression(value))?;
                }
                emit!(self, Opcode::OpHash, [pairs.len() as u64 * 2]);
            }
            Expression::Array(elements) => {
                for element in elements {
                    self.compile_node(&Node::Expression(element))?;
                }
                emit!(self, Opcode::OpArray, [elements.len() as u64]);
            }
            Expression::Infix(op, left, right) => {
                if op == &token!(<) {
                    // Reverse the order of the operands
                    self.compile_node(&Node::Expression(right))?;
                    self.compile_node(&Node::Expression(left))?;
                    emit!(self, Opcode::OpGreaterThan);
                    return Ok(());
                }
                self.compile_node(&Node::Expression(left))?;
                self.compile_node(&Node::Expression(right))?;
                match op {
                    token!(+) => emit!(self, Opcode::OpAdd),
                    token!(-) => emit!(self, Opcode::OpSub),
                    token!(*) => emit!(self, Opcode::OpMul),
                    token!(/) => emit!(self, Opcode::OpDiv),
                    token!(==) => emit!(self, Opcode::OpEqual),
                    token!(!=) => emit!(self, Opcode::OpNotEqual),
                    token!(>) => emit!(self, Opcode::OpGreaterThan),
                    _ => Err(anyhow!("unknown operator: {}", op))?,
                };
            }
            Expression::Prefix(op, right) => {
                self.compile_node(&Node::Expression(right))?;
                match op {
                    token!(-) => emit!(self, Opcode::OpMinus),
                    token!(!) => emit!(self, Opcode::OpBang),
                    _ => Err(anyhow!("unknown operator: {}", op))?,
                };
            }
            Expression::If(condition, consequence, alternative) => {
                self.compile_node(&Node::Expression(condition))?;

                // Emit an OpJumpNotTruthy with a bogus value
                let jump_not_truthy_pos = emit!(self, Opcode::OpJumpNotTruthy, [9999]);

                self.compile_node(&Node::Statement(consequence))?;

                if self.last_instruction_is_pop() {
                    self.remove_last_pop();
                }

                // Emit an OpJump with a bogus value
                let jump_pos = emit!(self, Opcode::OpJump, [9999]);

                let after_consequence_pos = self.instructions.len();
                self.change_operand(jump_not_truthy_pos, after_consequence_pos as u64);

                if let Some(alternative) = alternative {
                    let after_consequence_pos = self.instructions.len();
                    self.change_operand(jump_not_truthy_pos, after_consequence_pos as u64);

                    self.compile_node(&Node::Statement(alternative))?;

                    if self.last_instruction_is_pop() {
                        self.remove_last_pop();
                    }
                } else {
                    emit!(self, Opcode::OpNull);
                }

                let after_alternative_pos = self.instructions.len();
                self.change_operand(jump_pos, after_alternative_pos as u64);
            }
            Expression::Identifier(name) => {
                let symbol = self
                    .symbol_table
                    .resolve(name)
                    .ok_or_else(|| anyhow!("undefined variable: {}", name))?;
                emit!(self, Opcode::OpGetGlobal, [symbol.index as u64]);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    // Add a constant to the compiler
    pub fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);
        return self.constants.len() - 1;
    }

    // Emit an instruction
    pub fn emit(&mut self, opcode: Opcode, operands: Vec<u64>) -> usize {
        let instruction = make(opcode, operands);
        let position = self.add_instruction(instruction);

        self.set_last_instruction(opcode, position);

        return position;
    }

    // Add an instruction to the compiler
    pub fn add_instruction(&mut self, instruction: Instructions) -> usize {
        let position = self.instructions.len();
        self.instructions.extend(instruction);
        return position;
    }

    // Set the last instruction
    pub fn set_last_instruction(&mut self, opcode: Opcode, position: usize) {
        let previous = self.last_instruction.clone();
        self.previous_instruction = previous;
        self.last_instruction = Some(EmittedInstruction { opcode, position });
    }

    // Check if the last instruction was a pop
    pub fn last_instruction_is_pop(&self) -> bool {
        if let Some(instruction) = &self.last_instruction {
            return instruction.opcode == Opcode::OpPop;
        }
        return false;
    }

    // Remove the last instruction
    pub fn remove_last_pop(&mut self) {
        let last = self.last_instruction.clone();
        self.last_instruction = self.previous_instruction.clone();

        if let Some(instruction) = last {
            self.instructions.truncate(instruction.position);
        }
    }

    // Replace an instruction
    pub fn replace_instruction(&mut self, position: usize, new_instruction: Instructions) {
        for (i, instruction) in new_instruction.iter().enumerate() {
            self.instructions[position + i] = *instruction;
        }
    }

    // Change the operand of an instruction
    pub fn change_operand(&mut self, position: usize, operand: u64) {
        let op = Opcode::try_from(self.instructions[position]).unwrap();
        let new_instruction = make(op, vec![operand]);
        self.replace_instruction(position, new_instruction);
    }

    // Get the compiled bytecode
    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        }
    }
}
