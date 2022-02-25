pub enum Operation {
    None,
    Push,
    Dump,

    Add,
    Sub,
    Mul,
    Div,
}

pub struct Instruction {
    pub op: Operation,
    pub value: Option<String>,
}

impl Instruction {
    pub fn new(op: Operation, value: Option<String>) -> Self {
        Self {
            op: op,
            value: value,
        }
    }
}
