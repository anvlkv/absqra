use super::*;
use ra_lexer::errors::LexerError;
use failure::{Fail, Backtrace};


#[derive(Debug, Fail)]
pub enum ParserError {
    #[fail(display = "{} Unexpected indentation level: {}, \n {}", _1, _0, _2)]
    UnexpectedIndentLevel(u16, Position, Backtrace),
    #[fail(display = "{} Unexpected {}, \n {}", _1, _0, _2)]
    UnexpectedToken(String, Position, Backtrace),
    #[fail(display = "{} Expected [{}] got {}, \n {}", _2, _0, _1, _3)]
    ExpectedAGotB(String, String, Position, Backtrace),
    #[fail(display = "Unexpected end of input at {}, \n {}", _0, _1)]
    UnexpectedEndOfInput(Position, Backtrace),
    #[fail(display = "Invalid expression at {}, \n {}", _0, _1)]
    InvalidExpression(Position, Backtrace),
    #[fail(display = "Invalid block {}", _0)]
    InvalidBlock(Backtrace),
    #[fail(display = "LexerError: {}, \n {}", _0, _1)]
    LexerError(#[cause] LexerError, Backtrace),
}