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
                "Signed values may not behave as expected {}:{}",
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

            escaped = ch == '\\' && !escaped
        }
    }

    Err(())
}

pub fn try_parse_char(word: &Token) -> Result<u64, ()> {
    let tok = word.token.as_str();

    if !tok.starts_with('\'') && !tok.ends_with('\'') || tok.len() > 4 || tok.len() <= 2 {
        return Err(());
    }

    if tok.len() == 4 && tok.as_bytes()[1] as char != '\\' {
        return Err(());
    }

    // Char is escaped
    if tok.len() == 4 {
        let ch = char_to_escape_code(tok.as_bytes()[2] as char)?;
        return Ok(ch as u64);
    }

    Ok(tok.as_bytes()[1] as u64)
}

fn char_to_escape_code(ch: char) -> Result<u8, ()> {
    match ch {
        '\'' => Ok(0x27),
        '"' => Ok(0x22),
        '?' => Ok(0x3f),
        '\\' => Ok(0x5c),
        'a' => Ok(0x07),
        'b' => Ok(0x08),
        'f' => Ok(0x0c),
        'n' => Ok(0x0a),
        'r' => Ok(0x0d),
        't' => Ok(0x09),
        'v' => Ok(0x0b),
        '0' => Ok(0x00),
        _ => {
            error_println!("\\{} is not a valid escape character", ch);
            Err(())
        }
    }
}

pub fn try_parse_identifier(word: &Token) -> Result<String, ()> {
    // Always return Ok for now
    Ok(word.token.to_string())
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
