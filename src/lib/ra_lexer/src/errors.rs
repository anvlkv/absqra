use std::error;
use std::fmt;
use std::num;
use super::Position;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedIndentLevel(u16, Position),
    UnexpectedCharacter(char, Position),
    UnsupportedToken(Position),
    UnexpectedEndOfInput(Position),
    UnexpectedEndOfLine(Position),
    InvalidFloat(num::ParseFloatError, Position),
    InvalidInt(num::ParseIntError, Position)
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let err_text = match self {
            LexerError::UnexpectedIndentLevel(lvl, pos) =>  format!("Unexpected indentation level: {} at {}", lvl, pos),
            LexerError::UnexpectedCharacter(ch, pos) =>  format!("Unexpected character: {} at {}", ch, pos),
            LexerError::UnsupportedToken(pos) =>  format!("Unsupported token at {}", pos),
            LexerError::UnexpectedEndOfInput(pos) =>  format!("Unexpected end of input at {}", pos),
            LexerError::UnexpectedEndOfLine(pos) =>  format!("Unexpected end of line at {}", pos),
            LexerError::InvalidFloat(e, pos) => format!("Invalid floating point number: {} at {}", e, pos),
            LexerError::InvalidInt(e, pos) => format!("Invalid number: {} at {}", e, pos),
        };

        write!(f, "Lexer error: {}", err_text)
     }
}

impl error::Error for LexerError {}