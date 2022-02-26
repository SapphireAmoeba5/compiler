use crate::{error_println, operation::*};

pub fn lex_source(source: &str) -> Result<Vec<Instruction>, ()> {
    let source: Vec<&str> = source.split_ascii_whitespace().collect();
    let mut tokens: Vec<Instruction> = Vec::new();

    let mut label_id: usize = 0;
    let mut if_stack: Vec<usize> = Vec::new();

    for instr in source.iter() {
        match *instr {
            "." => tokens.push(Instruction::new(Operation::Dump, None)),

            "+" => tokens.push(Instruction::new(Operation::Add, None)),
            "-" => tokens.push(Instruction::new(Operation::Sub, None)),
            "*" => tokens.push(Instruction::new(Operation::Mul, None)),
            "/" => tokens.push(Instruction::new(Operation::Div, None)),
            "%" => tokens.push(Instruction::new(Operation::Mod, None)),

            "=" => tokens.push(Instruction::new(Operation::Eq, None)),
            ">" => tokens.push(Instruction::new(Operation::GreaterThan, None)),
            ">=" => tokens.push(Instruction::new(Operation::GreaterThanEqual, None)),
            "<" => tokens.push(Instruction::new(Operation::LessThan, None)),
            "<=" => tokens.push(Instruction::new(Operation::LessThanEqual, None)),
            "!" => tokens.push(Instruction::new(Operation::Not, None)),

            "~" => tokens.push(Instruction::new(Operation::BitwiseNot, None)),
            "&" => tokens.push(Instruction::new(Operation::BitwiseAnd, None)),
            "|" => tokens.push(Instruction::new(Operation::BitwiseOr, None)),
            "^" => tokens.push(Instruction::new(Operation::BitwiseXor, None)),

            "if" => {
                tokens.push(Instruction::new(Operation::If, Some(label_id.to_string())));
                if_stack.push(label_id);
                label_id += 1;
            }
            "end" => {
                let id = match if_stack.pop() {
                    Some(t) => t,
                    None => {
                        error_println!("Homeless 'end'");
                        return Err(());
                    }
                };
                tokens.push(Instruction::new(Operation::End, Some(id.to_string())));
            }
            _ => {
                match instr.parse::<u64>() {
                    Ok(_) => {}
                    Err(_) => {
                        error_println!("{} is not recognizable syntax", instr);
                        return Err(());
                    }
                }
                tokens.push(Instruction::new(Operation::Push, Some(instr.to_string())));
            }
        }
    }
    Ok(tokens)
}
