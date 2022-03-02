#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    None,
    Push,
    Dump,
    Dupe,
    Pop,
    Swap,
    Over,
    Rot,
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
    And,
    Or,
    // Bitwise
    BitwiseNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    // Control flow
    If,
    Else,
    While,
    Do,
    End,
}

#[derive(Debug)]
pub struct Instruction {
    pub op: Operation,
    pub value: String,
}

impl Instruction {
    pub fn new(op: Operation, value: String) -> Self {
        Self {
            op: op,
            value: value,
        }
    }
}
