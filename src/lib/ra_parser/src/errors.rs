use ra_lexer::cursor::Position;
use ra_lexer::errors::{LexerError};
extern crate failure;
use failure::{Fail, Backtrace};


#[derive(Debug, Fail)]
pub enum ParserError {
    #[fail(display = "{} Unexpected indentation level: {}, \n {}", _1, _0, _2)]
    UnexpectedIndentLevel(u16, Position, Backtrace),
    #[fail(display = "{} Unexpected {}, \n {}", _1, _0, _2)]
    UnexpectedToken(String, Position, Backtrace),
    #[fail(display = "{} Expected [{}] got {}, \n {}", _2, _1, _0, _3)]
    ExpectedAGotB(String, String, Position, Backtrace),
    #[fail(display = "Unexpected end of input at {}, \n {}", _0, _1)]
    UnexpectedEndOfInput(Position, Backtrace),
    #[fail(display = "Invalid expression at {}, \n {}", _0, _1)]
    InvalidExpression(Position, Backtrace),
    #[fail(display = "Invalid block")]
    InvalidBlock,
    #[fail(display = "ContentParsingError: {}, \n {}", _0, _1)]
    ContentParsingError(#[cause] LexerError, Backtrace)
}

impl From<ParserError> for Vec<ParserError> {
    fn from(err: ParserError) -> Vec<ParserError> {
        vec![err]
    }
}

// impl<'a> error::Error for ParserError<'a> {}