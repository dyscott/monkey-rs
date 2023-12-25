use crate::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
    object::Object,
};
use anyhow::{anyhow, Result};

#[cfg(test)]
mod tests;

const STACK_SIZE: usize = 2048;
const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

#[derive(Debug, PartialEq, Clone)]
pub struct VM {
    pub constants: Vec<Object>,
    pub instructions: Instructions,
    pub stack: [Object; STACK_SIZE],
    pub sp: usize,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        VM {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: vec![Object::Null; STACK_SIZE].try_into().unwrap(),
            sp: 0,
        }
    }

    pub fn stack_top(&self) -> Object {
        if self.sp == 0 {
            return Object::Null;
        }
        self.stack[self.sp - 1].clone()
    }

    pub fn run(&mut self) -> Result<()> {
        let mut ip = 0;
        while ip < self.instructions.len() {
            let op = Opcode::try_from(self.instructions[ip])?;
            match op {
                Opcode::OpConstant => {
                    let const_index = read_u16(&self.instructions[ip + 1..ip + 3]);
                    ip += 2;
                    let constant = self.constants[const_index as usize].clone();
                    self.push(constant)?;
                }
                Opcode::OpPop => {
                    self.pop()?;
                }
                Opcode::OpAdd | Opcode::OpSub | Opcode::OpMul | Opcode::OpDiv => {
                    self.exec_binary_op(op)?;
                }
                Opcode::OpTrue => {
                    self.push(TRUE.clone())?;
                }
                Opcode::OpFalse => {
                    self.push(FALSE.clone())?;
                }
                Opcode::OpEqual | Opcode::OpNotEqual | Opcode::OpGreaterThan => {
                    self.exec_comparison(op)?;
                }
                Opcode::OpBang => {
                    self.exec_bang_op()?;
                }
                Opcode::OpMinus => {
                    self.exec_minus_op()?;
                }
                Opcode::OpJump => {
                    let pos = read_u16(&self.instructions[ip + 1..]) as usize;
                    ip = pos - 1;
                }
                Opcode::OpJumpNotTruthy => {
                    let pos = read_u16(&self.instructions[ip + 1..]) as usize;
                    ip += 2;
                    let condition = self.pop()?;
                    if !condition.is_truthy() {
                        ip = pos - 1;
                    }
                }
                Opcode::OpNull => {
                    self.push(NULL.clone())?;
                }
                _ => unimplemented!("opcode not implemented: {}", op)
            }
            ip += 1;
        }
        Ok(())
    }

    pub fn push(&mut self, obj: Object) -> Result<()> {
        if self.sp >= STACK_SIZE {
            return Err(anyhow!("stack overflow"));
        }
        self.stack[self.sp] = obj;
        self.sp += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Object> {
        if self.sp == 0 {
            return Err(anyhow!("stack underflow"));
        }
        self.sp -= 1;
        Ok(self.stack[self.sp].clone())
    }

    pub fn last_popped_stack_elem(&self) -> Object {
        return self.stack[self.sp].clone();
    }

    pub fn exec_binary_op(&mut self, op: Opcode) -> Result<()> {
        let right = self.pop()?;
        let left = self.pop()?;
        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => {
                self.exec_binary_int_op(op, left, right)
            }
            (left, right) => Err(anyhow!(
                "unsupported types for binary operation: {} {}",
                left,
                right
            )),
        }
    }

    pub fn exec_binary_int_op(&mut self, op: Opcode, left: i64, right: i64) -> Result<()> {
        let result = match op {
            Opcode::OpAdd => left + right,
            Opcode::OpSub => left - right,
            Opcode::OpMul => left * right,
            Opcode::OpDiv => left / right,
            _ => return Err(anyhow!("unknown integer operator: {}", op)),
        };
        return self.push(Object::Integer(result));
    }

    pub fn exec_comparison(&mut self, op: Opcode) -> Result<()> {
        let right = self.pop()?;
        let left = self.pop()?;
        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => {
                self.exec_comparison_int_op(op, left, right)
            },
            (Object::Boolean(left), Object::Boolean(right)) => {
                match op {
                    Opcode::OpEqual => self.push(Object::Boolean(left == right)),
                    Opcode::OpNotEqual => self.push(Object::Boolean(left != right)),
                    _ => Err(anyhow!("unknown boolean operator: {}", op)),
                }
            },
            (left, right) => Err(anyhow!(
                "unsupported types for comparison: {} {}",
                left,
                right
            )),
        }
    }

    pub fn exec_comparison_int_op(&mut self, op: Opcode, left: i64, right: i64) -> Result<()> {
        let result = match op {
            Opcode::OpEqual => left == right,
            Opcode::OpNotEqual => left != right,
            Opcode::OpGreaterThan => left > right,
            _ => return Err(anyhow!("unknown integer operator: {}", op)),
        };
        return self.push(Object::Boolean(result));
    }

    pub fn exec_bang_op(&mut self) -> Result<()> {
        let operand = self.pop()?;
        match operand {
            Object::Boolean(value) => self.push(Object::Boolean(!value)),
            Object::Null => self.push(TRUE.clone()),
            _ => self.push(FALSE.clone()),
        }
    }

    pub fn exec_minus_op(&mut self) -> Result<()> {
        let operand = self.pop()?;
        match operand {
            Object::Integer(value) => self.push(Object::Integer(-value)),
            _ => Err(anyhow!("unsupported type for negation: {}", operand)),
        }
    }
}