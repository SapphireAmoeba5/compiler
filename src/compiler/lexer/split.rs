use std::os::raw::c_ulong;

#[derive(Debug)]
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
    let mut column: usize = 1;
    let mut offset: usize = 0;

    let mut in_string = false;
    let mut escaped = false;
    for (i, ch) in source.chars().enumerate() {
        let blank = source[offset..i].trim().is_empty();

        if ch == '\n' {
            line_number += 1;
            column = 0;
        }

        column += 1;
        if ch.is_whitespace() && !in_string && !blank {
            let word = &source[offset..i];
            push_word(&mut tokens, word, line_number, column);
            offset = i;
        } else if ch == '"' && !escaped {
            in_string = !in_string;
        }

        escaped = ch == '\\' && in_string
    }
    // Dump the rest of the buffer
    push_word(&mut tokens, &source[offset..], line_number, column + 1);
    tokens
}

/// Only pushes the word if the word isn't only comprised of whitespace
fn push_word(tokens: &mut Vec<Token>, word: &str, line_number: usize, column: usize) {
    let word = word.trim();
    if !word.is_empty() {
        let begin_column = column - word.len();
        tokens.push(Token::new(word, line_number, begin_column));
    }
}
