pub mod symbol_table;

use std::cell::RefCell;
use std::rc::Rc;

use crate::code::{make, Instructions, Opcode};
use crate::lexer::token::Token;
use crate::object::{CompiledFunction, Object};
use crate::parser::ast::{Expression, Node, Program, Statement};
use crate::token;
use anyhow::{anyhow, Result};
use symbol_table::SymbolTable;

use self::symbol_table::{GLOBAL_SCOPE, LOCAL_SCOPE};

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
    constants: Vec<Object>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    scopes: Vec<CompilationScope>,
    scope_index: usize,
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

#[derive(Debug, PartialEq, Clone)]
struct CompilationScope {
    instructions: Instructions,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
}

impl Compiler {
    // Create a new compiler
    pub fn new() -> Self {
        let main_scope = CompilationScope {
            instructions: vec![],
            last_instruction: None,
            previous_instruction: None,
        };
        Self {
            constants: vec![],
            symbol_table: SymbolTable::new(None),
            scopes: vec![main_scope],
            scope_index: 0,
        }
    }

    // Reset compiler for reuse (for REPL)
    pub fn reset(&mut self) {
        let main_scope = CompilationScope {
            instructions: vec![],
            last_instruction: None,
            previous_instruction: None,
        };
        self.scopes = vec![main_scope];
        self.scope_index = 0;
    }

    // Enter a new scope
    fn enter_scope(&mut self) {
        let scope = CompilationScope {
            instructions: vec![],
            last_instruction: None,
            previous_instruction: None,
        };
        self.scopes.push(scope);
        self.scope_index += 1;

        self.symbol_table = SymbolTable::new(Some(self.symbol_table.clone()));
    }

    // Leave the current scope
    fn leave_scope(&mut self) -> Instructions {
        let scope = self.scopes.pop().unwrap();
        self.scope_index -= 1;

        let old_symbol_table = self.symbol_table.clone();
        self.symbol_table = match old_symbol_table.borrow().outer {
            Some(ref outer) => outer.clone(),
            None => old_symbol_table.clone(), // This should never happen
        };

        return scope.instructions;
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
    fn compile_program(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            self.compile_node(&Node::Statement(&statement))?;
        }
        Ok(())
    }

    // Compile a statement AST node
    fn compile_statement(&mut self, statement: &Statement) -> Result<()> {
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
                let symbol = self.symbol_table.borrow_mut().define(name);
                match symbol.scope {
                    GLOBAL_SCOPE => emit!(self, Opcode::OpSetGlobal, [symbol.index as u64]),
                    LOCAL_SCOPE => emit!(self, Opcode::OpSetLocal, [symbol.index as u64]),
                    _ => Err(anyhow!("unknown scope: {}", symbol.scope))?,
                };
            }
            Statement::Return(expression) => {
                self.compile_node(&Node::Expression(expression))?;
                emit!(self, Opcode::OpReturnValue);
            }
        }
        Ok(())
    }

    // Compile an expression AST node
    fn compile_expression(&mut self, expression: &Expression) -> Result<()> {
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

                if self.last_instruction_is(Opcode::OpPop) {
                    self.remove_last_pop();
                }

                // Emit an OpJump with a bogus value
                let jump_pos = emit!(self, Opcode::OpJump, [9999]);

                let after_consequence_pos = self.current_instructions().len();
                self.change_operand(jump_not_truthy_pos, after_consequence_pos as u64);

                if let Some(alternative) = alternative {
                    let after_consequence_pos = self.current_instructions().len();
                    self.change_operand(jump_not_truthy_pos, after_consequence_pos as u64);

                    self.compile_node(&Node::Statement(alternative))?;

                    if self.last_instruction_is(Opcode::OpPop) {
                        self.remove_last_pop();
                    }
                } else {
                    emit!(self, Opcode::OpNull);
                }

                let after_alternative_pos = self.current_instructions().len();
                self.change_operand(jump_pos, after_alternative_pos as u64);
            }
            Expression::Identifier(name) => {
                let symbol = self
                    .symbol_table
                    .borrow()
                    .resolve(name)
                    .ok_or_else(|| anyhow!("undefined variable: {}", name))?;
                if symbol.scope == GLOBAL_SCOPE {
                    emit!(self, Opcode::OpGetGlobal, [symbol.index as u64]);
                } else {
                    emit!(self, Opcode::OpGetLocal, [symbol.index as u64]);
                }
            }
            Expression::Index(left, index) => {
                self.compile_node(&Node::Expression(left))?;
                self.compile_node(&Node::Expression(index))?;
                emit!(self, Opcode::OpIndex);
            }
            Expression::SliceIndex(left, start, end) => {
                self.compile_node(&Node::Expression(left))?;
                match start {
                    Some(start) => self.compile_node(&Node::Expression(start))?,
                    None => {
                        emit!(self, Opcode::OpNull);
                    }
                };
                match end {
                    Some(end) => self.compile_node(&Node::Expression(end))?,
                    None => {
                        emit!(self, Opcode::OpNull);
                    }
                };
                emit!(self, Opcode::OpSliceIndex);
            }
            Expression::Function(_, body) => {
                self.enter_scope();

                self.compile_node(&Node::Statement(body))?;

                if self.last_instruction_is(Opcode::OpPop) {
                    self.replace_last_pop_with_return()?;
                }
                if !self.last_instruction_is(Opcode::OpReturnValue) {
                    emit!(self, Opcode::OpReturn);
                }
                
                let num_locals = self.symbol_table.borrow().num_definitions;
                let instructions = self.leave_scope();

                let compiled_fn = Object::CompiledFunction(CompiledFunction { instructions, num_locals });
                let constant = self.add_constant(compiled_fn);
                emit!(self, Opcode::OpConstant, [constant as u64]);
            }
            Expression::Call(function, _) => {
                self.compile_node(&Node::Expression(function))?;

                emit!(self, Opcode::OpCall);
            }
        }
        Ok(())
    }

    // Add a constant to the compiler
    fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);
        return self.constants.len() - 1;
    }

    // Emit an instruction
    fn emit(&mut self, opcode: Opcode, operands: Vec<u64>) -> usize {
        let instruction = make(opcode, operands);
        let position = self.add_instruction(instruction);

        self.set_last_instruction(opcode, position);

        return position;
    }

    // Get instructions from the current scope
    fn current_instructions(&self) -> &Instructions {
        &self.scopes[self.scope_index].instructions
    }

    // Get instructions from the current scope (mutable)
    fn current_instructions_mut(&mut self) -> &mut Instructions {
        &mut self.scopes[self.scope_index].instructions
    }

    // Add an instruction to the compiler
    fn add_instruction(&mut self, instruction: Instructions) -> usize {
        let position = self.current_instructions().len();
        self.current_instructions_mut().extend(instruction);
        return position;
    }

    // Set the last instruction
    fn set_last_instruction(&mut self, opcode: Opcode, position: usize) {
        let previous = self.scopes[self.scope_index].last_instruction.clone();
        self.scopes[self.scope_index].previous_instruction = previous;
        self.scopes[self.scope_index].last_instruction =
            Some(EmittedInstruction { opcode, position });
    }

    // Check if the last instruction was a pop
    fn last_instruction_is(&self, opcode: Opcode) -> bool {
        if let Some(instruction) = &self.scopes[self.scope_index].last_instruction {
            return instruction.opcode == opcode;
        }
        return false;
    }

    // Remove the last instruction
    fn remove_last_pop(&mut self) {
        let last = self.scopes[self.scope_index].last_instruction.clone();
        let previous = self.scopes[self.scope_index].previous_instruction.clone();

        if let Some(instruction) = last {
            self.current_instructions_mut()
                .truncate(instruction.position);
            self.scopes[self.scope_index].last_instruction = previous;
        }
    }

    // Replace an instruction
    fn replace_instruction(&mut self, position: usize, new_instruction: Instructions) {
        for (i, instruction) in new_instruction.iter().enumerate() {
            self.current_instructions_mut()[position + i] = *instruction;
        }
    }

    // Replace the last instruction with a return
    fn replace_last_pop_with_return(&mut self) -> Result<()> {
        let last_pos = match &self.scopes[self.scope_index].last_instruction {
            Some(instruction) => instruction.position,
            None => Err(anyhow!("no last instruction to replace"))?,
        };

        self.replace_instruction(last_pos, make(Opcode::OpReturnValue, vec![]));

        self.scopes[self.scope_index].last_instruction = Some(EmittedInstruction {
            opcode: Opcode::OpReturnValue,
            position: last_pos,
        });
        Ok(())
    }

    // Change the operand of an instruction
    fn change_operand(&mut self, position: usize, operand: u64) {
        let op = Opcode::try_from(self.current_instructions()[position]).unwrap();
        let new_instruction = make(op, vec![operand]);
        self.replace_instruction(position, new_instruction);
    }

    // Get the compiled bytecode
    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.current_instructions().clone(),
            constants: self.constants.clone(),
        }
    }
}
