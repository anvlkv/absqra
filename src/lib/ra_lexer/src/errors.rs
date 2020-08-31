use std::num;
use crate::cursor::Position;

extern crate failure;
use failure::Fail;


#[derive(Debug, Fail, PartialEq)]
pub enum LexerError {
    #[fail(display = "Unexpected indentation level: {} at {}", _0, _1)]
    UnexpectedIndentLevel(u16, Position),
    #[fail(display = "Unexpected character: {} at {}", _0, _1)]
    UnexpectedCharacter(char, Position),
    #[fail(display = "Unsupported token at {}", _0)]
    UnsupportedToken(Position),
    #[fail(display = "Unexpected end of input at {}", _0)]
    UnexpectedEndOfInput(Position),
    #[fail(display = "Unexpected end of line at {}", _0)]
    UnexpectedEndOfLine(Position),
    #[fail(display = "Invalid floating point number: {} at {}", _0, _1)]
    InvalidFloat(#[cause] num::ParseFloatError, Position),
    #[fail(display = "Invalid number: {} at {}", _0, _1)]
    InvalidInt(#[cause] num::ParseIntError, Position)
}
