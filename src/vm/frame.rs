use crate::{code::Instructions, object::CompiledFunction};

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    function: CompiledFunction,
    pub ip: usize,
}

impl Frame {
    pub fn new(function: CompiledFunction) -> Self {
        Self {
            function,
            ip: 0,
        }
    }
    pub fn instructions(&self) -> &Instructions {
        &self.function.instructions
    }
}
