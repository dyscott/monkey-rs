mod frame;

use std::collections::HashMap;

use crate::{
    code::{read_u16, Opcode},
    compiler::Bytecode,
    object::{
        builtins::{get_builtin, BUILTINS},
        BuiltInFunction, Closure, CompiledFunction, HashKey, Object,
    },
};
use anyhow::{anyhow, Result};
use frame::Frame;

#[cfg(test)]
mod tests;

const STACK_SIZE: usize = 2048;
const GLOBALS_SIZE: usize = 65536;
const MAX_FRAMES: usize = 1024;
const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

#[derive(Debug, PartialEq, Clone)]
pub struct VM {
    pub constants: Vec<Object>,
    pub stack: Vec<Object>,
    pub sp: usize,
    pub globals: Vec<Object>,

    frames: Vec<Frame>,
    frames_index: usize,
}

impl Default for VM {
    fn default() -> Self {
        VM {
            constants: vec![],
            stack: vec![Object::Null; STACK_SIZE],
            sp: 0,
            globals: vec![Object::Null; GLOBALS_SIZE],

            frames: vec![],
            frames_index: 1,
        }
    }
}

impl VM {
    // Create a new VM
    pub fn new(bytecode: Bytecode) -> Self {
        let main_fn = CompiledFunction {
            instructions: bytecode.instructions,
            num_locals: 0,
            num_parameters: 0,
        };
        let main_closure = Closure {
            func: main_fn,
            free: vec![],
        };
        let main_frame = Frame::new(main_closure, 0);
        let mut frames = Vec::with_capacity(MAX_FRAMES);
        frames.push(main_frame);

        VM {
            constants: bytecode.constants,
            stack: vec![Object::Null; STACK_SIZE],
            sp: 0,
            globals: vec![Object::Null; GLOBALS_SIZE],

            frames: frames,
            frames_index: 1,
        }
    }

    // Reset the VM for reuse (for REPL)
    pub fn reset(&mut self, bytecode: Bytecode) {
        let main_fn = CompiledFunction {
            instructions: bytecode.instructions,
            num_locals: 0,
            num_parameters: 0,
        };
        let main_closure = Closure {
            func: main_fn,
            free: vec![],
        };
        let main_frame = Frame::new(main_closure, 0);

        self.frames = vec![main_frame];
        self.frames_index = 1;
        self.constants = bytecode.constants;
        self.stack = vec![Object::Null; STACK_SIZE];
        self.sp = 0;
    }

    // Push an element onto the stack
    fn push(&mut self, obj: Object) -> Result<()> {
        if self.sp >= STACK_SIZE {
            return Err(anyhow!("stack overflow"));
        }
        self.stack[self.sp] = obj;
        self.sp += 1;
        Ok(())
    }

    // Pop an element from the stack
    fn pop(&mut self) -> Result<Object> {
        if self.sp == 0 {
            return Err(anyhow!("stack underflow"));
        }
        self.sp -= 1;
        Ok(self.stack[self.sp].clone())
    }

    // Get the last element popped from the stack
    pub fn last_popped_stack_elem(&self) -> Object {
        return self.stack[self.sp].clone();
    }

    // Get the top element of the stack
    pub fn stack_top(&self) -> Object {
        if self.sp == 0 {
            return Object::Null;
        }
        self.stack[self.sp - 1].clone()
    }

    // Get the current frame
    pub fn current_frame(&mut self) -> &mut Frame {
        &mut self.frames[self.frames_index - 1]
    }

    // Push a frame onto the stack
    pub fn push_frame(&mut self, frame: Frame) -> Result<()> {
        if self.frames_index >= MAX_FRAMES {
            return Err(anyhow!("frame overflow"));
        }
        self.frames.push(frame);
        self.frames_index += 1;
        Ok(())
    }

    // Pop a frame from the stack
    pub fn pop_frame(&mut self) -> Result<Frame> {
        self.frames_index -= 1;
        self.frames.pop().ok_or(anyhow!("frame underflow"))
    }

    // Run the VM
    pub fn run(&mut self) -> Result<()> {
        while self.current_frame().ip < self.current_frame().instructions().len() {
            let mut ip = self.current_frame().ip;
            let ins = self.current_frame().instructions();

            let op = Opcode::try_from(ins[ip])?;
            match op {
                Opcode::OpConstant => {
                    let const_index = read_u16(&ins[ip + 1..ip + 3]);
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
                    let pos = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip = pos - 1;
                }
                Opcode::OpJumpNotTruthy => {
                    let pos = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2;
                    let condition = self.pop()?;
                    if !condition.is_truthy() {
                        ip = pos - 1;
                    }
                }
                Opcode::OpNull => {
                    self.push(NULL.clone())?;
                }
                Opcode::OpSetGlobal => {
                    let global_index = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2;
                    self.globals[global_index] = self.pop()?;

                    self.stack[self.sp] = Object::Null;
                }
                Opcode::OpGetGlobal => {
                    let global_index = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2;
                    self.push(self.globals[global_index].clone())?;
                }
                Opcode::OpArray => {
                    let num_elements = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2;
                    let array = self.build_array(self.sp - num_elements, self.sp)?;
                    self.sp -= num_elements;
                    self.push(array)?;
                }
                Opcode::OpHash => {
                    let num_elements = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2;
                    let hash = self.build_hash(self.sp - num_elements, self.sp)?;
                    self.sp -= num_elements;
                    self.push(hash)?;
                }
                Opcode::OpIndex => {
                    let index = self.pop()?;
                    let left = self.pop()?;
                    self.exec_index_op(left, index)?;
                }
                Opcode::OpSliceIndex => {
                    let stop = self.pop()?;
                    let start = self.pop()?;
                    let left = self.pop()?;
                    self.exec_slice_index_op(left, start, stop)?;
                }
                Opcode::OpCall => {
                    let num_args = ins[ip + 1] as usize;
                    self.current_frame().ip += 1;

                    self.exec_call(num_args)?;

                    continue;
                }
                Opcode::OpReturnValue => {
                    let return_value = self.pop()?;

                    let frame = self.pop_frame()?;
                    ip = self.current_frame().ip;
                    self.sp = frame.base_pointer - 1;

                    self.push(return_value)?;
                }
                Opcode::OpReturn => {
                    let frame = self.pop_frame()?;
                    ip = self.current_frame().ip;
                    self.sp = frame.base_pointer - 1;

                    self.push(NULL.clone())?;
                }
                Opcode::OpSetLocal => {
                    let local_index = ins[ip + 1] as usize;
                    ip += 1;

                    let frame_base_pointer = self.current_frame().base_pointer;

                    self.stack[frame_base_pointer + local_index] = self.pop()?;
                    
                    self.stack[self.sp] = Object::Null;
                }
                Opcode::OpGetLocal => {
                    let local_index = ins[ip + 1] as usize;
                    ip += 1;

                    let frame_base_pointer = self.current_frame().base_pointer;

                    self.push(self.stack[frame_base_pointer + local_index].clone())?;
                }
                Opcode::OpGetBuiltin => {
                    let builtin_index = ins[ip + 1] as usize;
                    ip += 1;

                    let builtin = get_builtin(&BUILTINS[builtin_index].to_string())
                        .ok_or(anyhow!("builtin not found: {}", builtin_index))?;
                    self.push(Object::BuiltInFunction(builtin))?;
                }
                Opcode::OpClosure => {
                    let const_index = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    let num_free = ins[ip + 3] as usize;
                    ip += 3;

                    self.push_closure(const_index, num_free)?;
                }
                Opcode::OpGetFree => {
                    let free_index = ins[ip + 1] as usize;
                    ip += 1;

                    let current_closure = self.current_frame().cl.clone();
                    self.push(current_closure.free[free_index].clone())?;
                }
                Opcode::OpCurrentClosure => {
                    let current_closure = self.current_frame().cl.clone();
                    self.push(Object::Closure(current_closure))?;
                }
            }
            self.current_frame().ip = ip + 1;
        }
        Ok(())
    }

    // Execute a binary operator
    fn exec_binary_op(&mut self, op: Opcode) -> Result<()> {
        let right = self.pop()?;
        let left = self.pop()?;
        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => {
                self.exec_binary_int_op(op, left, right)
            }
            (Object::String(left), Object::String(right)) => {
                self.exec_binary_string_op(op, left, right)
            }
            (left, right) => Err(anyhow!(
                "unsupported types for binary operation: {} {}",
                left.type_name(),
                right.type_name()
            )),
        }
    }

    // Execute a binary operator on two integers
    fn exec_binary_int_op(&mut self, op: Opcode, left: i64, right: i64) -> Result<()> {
        let result = match op {
            Opcode::OpAdd => left + right,
            Opcode::OpSub => left - right,
            Opcode::OpMul => left * right,
            Opcode::OpDiv => left / right,
            _ => return Err(anyhow!("unknown integer operator: {}", op)),
        };
        return self.push(Object::Integer(result));
    }

    // Execute a binary operator on two strings
    fn exec_binary_string_op(&mut self, op: Opcode, left: String, right: String) -> Result<()> {
        let result = match op {
            Opcode::OpAdd => left + &right,
            _ => return Err(anyhow!("unknown string operator: {}", op)),
        };
        return self.push(Object::String(result));
    }

    // Execute a comparison operator
    pub fn exec_comparison(&mut self, op: Opcode) -> Result<()> {
        let right = self.pop()?;
        let left = self.pop()?;
        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => {
                self.exec_comparison_int_op(op, left, right)
            }
            (Object::Boolean(left), Object::Boolean(right)) => match op {
                Opcode::OpEqual => self.push(Object::Boolean(left == right)),
                Opcode::OpNotEqual => self.push(Object::Boolean(left != right)),
                _ => Err(anyhow!("unknown boolean operator: {}", op)),
            },
            (left, right) => Err(anyhow!(
                "unsupported types for comparison: {} {}",
                left.type_name(),
                right.type_name()
            )),
        }
    }

    // Execute a comparison operator on two integers
    fn exec_comparison_int_op(&mut self, op: Opcode, left: i64, right: i64) -> Result<()> {
        let result = match op {
            Opcode::OpEqual => left == right,
            Opcode::OpNotEqual => left != right,
            Opcode::OpGreaterThan => left > right,
            _ => return Err(anyhow!("unknown integer operator: {}", op)),
        };
        return self.push(Object::Boolean(result));
    }

    // Execute the prefix bang operator
    fn exec_bang_op(&mut self) -> Result<()> {
        let operand = self.pop()?;
        match operand {
            Object::Boolean(value) => self.push(Object::Boolean(!value)),
            Object::Null => self.push(TRUE.clone()),
            _ => self.push(FALSE.clone()),
        }
    }

    // Execute the prefix minus operator
    fn exec_minus_op(&mut self) -> Result<()> {
        let operand = self.pop()?;
        match operand {
            Object::Integer(value) => self.push(Object::Integer(-value)),
            _ => Err(anyhow!("unsupported type for negation: {}", operand)),
        }
    }

    // Build an array from the stack
    fn build_array(&mut self, start_index: usize, end_index: usize) -> Result<Object> {
        let mut elements = vec![];
        self.stack[start_index..end_index].iter().for_each(|elem| {
            elements.push(elem.clone());
        });
        return Ok(Object::Array(elements));
    }

    // Build a hash from the stack
    fn build_hash(&mut self, start_index: usize, end_index: usize) -> Result<Object> {
        let mut pairs = HashMap::new();
        let result = self.stack[start_index..end_index]
            .chunks_exact(2)
            .try_for_each(|chunk| {
                let key = chunk[0].clone();
                let value = chunk[1].clone();

                let key_type = key.type_name();
                let key: HashKey = match key.into() {
                    Some(key) => key,
                    None => return Err(anyhow!("unusable as hash key: {}", key_type)),
                };
                pairs.insert(key, value);
                Ok(())
            });
        match result {
            Err(err) => Err(err),
            Ok(_) => Ok(Object::Hash(pairs)),
        }
    }

    // Execute the index operator
    fn exec_index_op(&mut self, left: Object, index: Object) -> Result<()> {
        match (left, index) {
            (Object::Array(elements), Object::Integer(index)) => {
                self.exec_array_index(elements, index)
            }
            (Object::Hash(pairs), index) => self.exec_hash_index(pairs, index),
            (Object::String(string), Object::Integer(index)) => {
                self.exec_string_index(string, index)
            }
            (left, index) => Err(anyhow!(
                "index operator not supported: {}[{}]",
                left.type_name(),
                index.type_name()
            )),
        }
    }

    // Execute the index operator on an array
    fn exec_array_index(&mut self, elements: Vec<Object>, index: i64) -> Result<()> {
        let index = if index < 0 {
            (elements.len() as i64) + index
        } else {
            index
        };

        match elements.get(index as usize) {
            Some(elem) => self.push(elem.clone()),
            None => self.push(NULL.clone()),
        }
    }

    // Execute the index operator on a hash
    fn exec_hash_index(&mut self, pairs: HashMap<HashKey, Object>, index: Object) -> Result<()> {
        let index_type = index.type_name();
        let key = match index.into() {
            Some(key) => key,
            None => return Err(anyhow!("unusable as hash key: {}", index_type)),
        };
        match pairs.get(&key) {
            Some(value) => self.push(value.clone()),
            None => self.push(NULL.clone()),
        }
    }

    // Execute the index operator on a string
    fn exec_string_index(&mut self, string: String, index: i64) -> Result<()> {
        let index = if index < 0 {
            (string.len() as i64) + index
        } else {
            index
        };

        match string.chars().nth(index as usize) {
            Some(ch) => self.push(Object::String(ch.to_string())),
            None => self.push(NULL.clone()),
        }
    }

    // Execute the slice index operator
    fn exec_slice_index_op(&mut self, left: Object, start: Object, stop: Object) -> Result<()> {
        match (left, start, stop) {
            (Object::Array(elements), start, stop) => {
                self.exec_array_slice_index(elements, start, stop)
            }
            (Object::String(string), start, stop) => {
                self.exec_string_slice_index(string, start, stop)
            }
            (left, start, stop) => Err(anyhow!(
                "slice index operator not supported: {}[{}:{}]",
                left.type_name(),
                start.type_name(),
                stop.type_name()
            )),
        }
    }

    // Execute the slice index operator on an array
    fn exec_array_slice_index(
        &mut self,
        elements: Vec<Object>,
        start: Object,
        stop: Object,
    ) -> Result<()> {
        let start = match start {
            Object::Integer(start) => start,
            Object::Null => 0,
            _ => return Err(anyhow!("slice start must be an integer")),
        };
        let stop = match stop {
            Object::Integer(stop) => stop,
            Object::Null => elements.len() as i64,
            _ => return Err(anyhow!("slice stop must be an integer")),
        };

        let start = if start < 0 {
            (elements.len() as i64) + start
        } else if start > (elements.len() as i64) {
            elements.len() as i64
        } else {
            start
        };
        let stop = if stop < 0 {
            (elements.len() as i64) + stop
        } else if stop > (elements.len() as i64) {
            elements.len() as i64
        } else {
            stop
        };

        match elements.get(start as usize..stop as usize) {
            Some(slice) => self.push(Object::Array(slice.to_vec()))?,
            None => self.push(Object::Array(vec![]))?,
        };
        Ok(())
    }

    // Execute the slice index operator on a string
    fn exec_string_slice_index(
        &mut self,
        string: String,
        start: Object,
        stop: Object,
    ) -> Result<()> {
        let start = match start {
            Object::Integer(start) => start,
            Object::Null => 0,
            _ => return Err(anyhow!("slice start must be an integer")),
        };
        let stop = match stop {
            Object::Integer(stop) => stop,
            Object::Null => string.len() as i64,
            _ => return Err(anyhow!("slice stop must be an integer")),
        };

        let start = if start < 0 {
            (string.len() as i64) + start
        } else if start > (string.len() as i64) {
            string.len() as i64
        } else {
            start
        };
        let stop = if stop < 0 {
            (string.len() as i64) + stop
        } else if stop > (string.len() as i64) {
            string.len() as i64
        } else {
            stop
        };

        match string.get(start as usize..stop as usize) {
            Some(slice) => self.push(Object::String(slice.to_string()))?,
            None => self.push(Object::String("".to_string()))?,
        };
        Ok(())
    }

    // Execute a function call
    fn exec_call(&mut self, num_args: usize) -> Result<()> {
        let callee = self.stack[self.sp - 1 - num_args].clone();
        match callee {
            Object::Closure(cl) => self.call_closure(cl, num_args),
            Object::BuiltInFunction(builtin_fn) => self.call_builtin(builtin_fn, num_args),
            _ => Err(anyhow!("calling non-closure and non-builtin")),
        }
    }

    // Push a closure onto the stack
    fn push_closure(&mut self, const_index: usize, num_free: usize) -> Result<()> {
        let constant = self.constants[const_index].clone();

        if let Object::CompiledFunction(func) = constant {
            let mut free = vec![];
            for i in 0..num_free {
                free.push(self.stack[self.sp - num_free + i].clone());
            }
            self.sp -= num_free;

            let cl = Closure { func, free };
            self.push(Object::Closure(cl))?;

            return Ok(());
        }

        return Err(anyhow!("not a function: {:?}", constant));
    }

    // Call a closure
    fn call_closure(&mut self, cl: Closure, num_args: usize) -> Result<()> {
        let num_locals = cl.func.num_locals;
        let num_params = cl.func.num_parameters;

        if num_args != num_params {
            return Err(anyhow!(
                "wrong number of arguments: want={}, got={}",
                num_params,
                num_args
            ));
        }

        let frame = Frame::new(cl, self.sp - num_args);
        self.sp = frame.base_pointer + num_locals;
        self.push_frame(frame)?;
        Ok(())
    }

    // Call a built-in function
    fn call_builtin(&mut self, builtin: BuiltInFunction, num_args: usize) -> Result<()> {
        let args = self.stack[self.sp - num_args..self.sp].to_vec();
        self.current_frame().ip += 1;

        let result = builtin(args)?;
        self.sp = self.sp - num_args - 1;
        self.push(result)?;
        Ok(())
    }
}
