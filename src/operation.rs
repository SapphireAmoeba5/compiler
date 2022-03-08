#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    None,
    // Stack pushing
    Push,
    PushString,

    // Compiler instrinsic
    Dump,
    Dupe,
    Pop,
    Swap,
    Over,
    Rot,
    Putc,
    Puts,
    Strlen,

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

    // Others
    Identifier,
    Func,
    Arrow,
    In,
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
