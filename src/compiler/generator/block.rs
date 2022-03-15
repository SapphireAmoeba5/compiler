use crate::operation::{Instruction, Operation};

pub struct Block {
    pub operation: Operation,
    pub block_id: usize,
}

impl Block {
    pub fn new(op: Operation, id: usize) -> Self {
        Self {
            operation: op,
            block_id: id,
        }
    }
}
