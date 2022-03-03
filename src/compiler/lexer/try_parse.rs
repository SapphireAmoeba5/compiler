use crate::compiler::lexer::*;
use crate::{debug_println, error_println, info_println, operation::*, warn_println};

pub fn try_parse_number(word: &Token) -> Result<String, ()> {
    let number = word.token.as_str();

    match number.parse::<u64>() {
        Ok(_) => {
            return Ok(number.to_string());
        }
        Err(_) => {}
    }

    match number.parse::<i64>() {
        Ok(_) => {
            warn_println!(
                "This language currently doesn't support signed values {}:{}",
                word.line_number,
                word.column
            );
            return Ok(number.to_string());
        }
        Err(_) => {}
    }

    match try_parse_hex(word) {
        Ok(s) => {
            return Ok(s);
        }
        Err(_) => {}
    }

    match try_parse_binary(word) {
        Ok(s) => {
            return Ok(s);
        }
        Err(_) => {}
    }

    Err(())
}

pub fn try_parse_string(word: &Token) -> Result<String, ()> {
    let string = word.token.as_str();

    if string.starts_with('"') && string.ends_with('"') {
        let mut escaped = false;

        for (i, ch) in string[1..].chars().enumerate() {
            if ch == '"' && !escaped {
                if string[..=i].len() + 1 != string.len() {
                    return Err(());
                }
                return Ok(string.to_string());
            }

            escaped = ch == '\\'
        }
    }

    Err(())
}

fn try_parse_hex(word: &Token) -> Result<String, ()> {
    if word.token.len() < 2 || !word.token.starts_with("0x") {
        return Err(());
    }

    match u64::from_str_radix(&word.token[2..], 16) {
        Ok(s) => {
            return Ok(s.to_string());
        }
        Err(_) => {
            return Err(());
        }
    }
}

fn try_parse_binary(word: &Token) -> Result<String, ()> {
    if word.token.len() < 2 || !word.token.starts_with("0b") {
        return Err(());
    }

    match u64::from_str_radix(&word.token[2..], 2) {
        Ok(s) => {
            return Ok(s.to_string());
        }
        Err(_) => {
            return Err(());
        }
    }
}
