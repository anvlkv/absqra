use std::num;
use failure::{Fail};
use super::*;


#[derive(Debug, Fail)]
pub enum LexerError {
    #[fail(display = "Unexpected indentation level: {} at {} {}", _0, _1, _2)]
    UnexpectedIndentLevel(u16, Position, Backtrace),
    #[fail(display = "Unexpected character: {} at {} {}", _0, _1, _2)]
    UnexpectedCharacter(char, Position, Backtrace),
    #[fail(display = "Unsupported token at {} {}", _0, _1)]
    UnsupportedToken(Position, Backtrace),
    #[fail(display = "Unexpected end of input at {} {}", _0, _1)]
    UnexpectedEndOfInput(Position, Backtrace),
    #[fail(display = "Unexpected end of line at {} {}", _0, _1)]
    UnexpectedEndOfLine(Position, Backtrace),
    #[fail(display = "Invalid floating point number: {} at {}", _0, _1)]
    InvalidFloat(#[cause] num::ParseFloatError, Position),
    #[fail(display = "Invalid number: {} at {}", _0, _1)]
    InvalidInt(#[cause] num::ParseIntError, Position)
}
