use anyhow::Result;
use crate::{code::{Instructions, Opcode, read_u16}, object::Object, compiler::Bytecode};

#[cfg(test)]
mod tests;

const STACK_SIZE: usize = 2048;

#[derive(Debug, PartialEq, Clone)]
pub struct VM {
    pub constants: Vec<Object>,
    pub instructions: Instructions,
    pub stack: Vec<Object>,
    pub sp: usize,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        VM {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: vec![Object::Null; STACK_SIZE],
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
                    self.push(constant);
                }
            }
            ip += 1;
        }
        Ok(())
    }

    pub fn push(&mut self, obj: Object) {
        if self.sp >= STACK_SIZE {
            panic!("stack overflow");
        }
        self.stack[self.sp] = obj;
        self.sp += 1;
    }
}
