use std::os::raw::c_ulong;

pub struct Token {
    pub token: String,
    pub line_number: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token: &str, line_number: usize, column: usize) -> Self {
        Self {
            token: token.to_string(),
            line_number: line_number,
            column: column,
        }
    }
}

pub fn split_tokens(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut line_number: usize = 1;
    let mut offset: usize = 0;

    let mut in_string = false;
    let mut escaped = false;

    for (column, ch) in source.chars().enumerate() {
        if ch.is_whitespace() && !in_string {
            tokens.push(Token::new(
                source[offset..column].trim(),
                line_number,
                offset + 1,
            ));
            offset = column;
            if ch == '\n' {
                line_number += 1;
            }
        } else if ch == '"' && !escaped {
            in_string = !in_string;
        }

        escaped = ch == '\\' && in_string;
    }

    tokens
}
