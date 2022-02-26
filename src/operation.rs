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
    pub values: Option<Vec<String>>,
}

impl Instruction {
    pub fn new(op: Operation, values: Option<Vec<String>>) -> Self {
        Self {
            op: op,
            values: values,
        }
    }
}
