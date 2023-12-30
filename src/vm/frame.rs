use crate::{code::Instructions, object::CompiledFunction};

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    function: CompiledFunction,
    pub ip: usize,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new(function: CompiledFunction, base_pointer: usize) -> Self {
        Self {
            function,
            ip: 0,
            base_pointer: base_pointer,
        }
    }
    pub fn instructions(&self) -> &Instructions {
        &self.function.instructions
    }
}
