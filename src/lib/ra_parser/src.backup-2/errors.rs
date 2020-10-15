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
    // #[fail(display = "Error chain: {}, caused by: {}", _1, _0)]
    // ChainedError(#[cause] Box<ParserError>, Box<ParserError>)
}


impl From<ParserError> for Vec<ParserError> {
    fn from(err: ParserError) -> Vec<ParserError> {
        vec![err]
    }
}

impl From<LexerError> for ParserError {
    fn from(err: LexerError) -> ParserError {
        ParserError::LexerError(err, Backtrace::new())
    }
}

// impl std::error::Error for Box<ParserError> {
    
// }

// impl<'a> error::Error for ParserError<'a> {}