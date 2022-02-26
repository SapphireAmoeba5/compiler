#[derive(Debug)]
pub enum Operation {
    None,
    Push,
    Dump,

    // Mathematical operations
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Conditional operators
    Eq,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Not,
    // Bitwise
    BitwiseNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    // Control flow
    If,
    Else,
    End,
}

#[derive(Debug)]
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
