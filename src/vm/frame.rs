use crate::{code::Instructions, object::Closure};

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub cl: Closure,
    pub ip: usize,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new(cl: Closure, base_pointer: usize) -> Self {
        Self {
            cl,
            ip: 0,
            base_pointer: base_pointer,
        }
    }
    pub fn instructions(&self) -> &Instructions {
        &self.cl.func.instructions
    }
}
