use std::fmt::Display;
use std::error;
use std::fmt;
use ra_lexer::token::{Token, TokenKind};
use ra_lexer::cursor::Position;
use ra_lexer::errors::{LexerError};
extern crate failure;
use failure::Fail;


#[derive(Debug)]
pub struct TokenKindExpectedOptions<'a>(Vec<TokenKind<'a>>);


#[derive(Debug, Fail)]
pub enum ParserError {
    #[fail(display = "{} Unexpected indentation level: {}", _1, _0)]
    UnexpectedIndentLevel(u16, Position),
    #[fail(display = "{} Unexpected {}", _1, _0)]
    UnexpectedToken(String, Position),
    #[fail(display = "{} Expected [{}] got {}", _2, _0, _1)]
    ExpectedAGotB(String, String, Position),
    #[fail(display = "Unexpected end of input at {}", _0)]
    UnexpectedEndOfInput(Position),
    #[fail(display = "Invalid expression at {}", _0)]
    InvalidExpression(Position),
    #[fail(display = "Invalid block")]
    InvalidBlock,
    #[fail(display = "ContentParsingError: {}", _0)]
    ContentParsingError(#[cause] LexerError)
}



// impl<'a> fmt::Display for ParserError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
//         let err_text = match self {
//             ParserError::UnexpectedIndentLevel(lvl, pos) =>  format!("Unexpected indentation level: {} at {}", lvl, pos),
//             ParserError::UnexpectedToken(token) =>  format!(, token.kind.unwrap(), token.content, token.position.0),
//             ParserError::UnexpectedEndOfInput(pos) =>  format!("Unexpected end of input at {}", pos),
//             ParserError::InvalidExpression(pos) =>  format!("Invalid expression at {}", pos),
//             ParserError::ExpectedAGotB(got, expected) => format!("Expected one of [{:?}], got {:?}", expected, got),
//             ParserError::InvalidBlock => format!("Invalid block"),
//             ParserError::ContentParsingError(err) => {
//                 return err.fmt(f)
//             }
//         };

//         write!(f, "Parser error: {}", err_text)
//      }
// }

// impl<'a> fmt::Debug for ParserError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
//         let err_text = match self {
//             ParserError::UnexpectedIndentLevel(lvl, pos) =>  format!("Unexpected indentation level: {} at {}", lvl, pos),
//             ParserError::UnexpectedToken(token) =>  format!("Unexpected {:?}: {} at {}", token.kind.unwrap(), token.content, token.position.0),
//             ParserError::UnexpectedEndOfInput(pos) =>  format!("Unexpected end of input at {}", pos),
//             ParserError::InvalidExpression(pos) =>  format!("Invalid expression at {}", pos),
//             ParserError::ExpectedAGotB(got, expected) => format!("Expected one of [{:?}], got {:?}", expected, got),
//             ParserError::InvalidBlock => format!("Invalid block"),
//             ParserError::ContentParsingError(err) => {
//                 return err.fmt(f)
//             }
//         };

//         write!(f, "{:?}", err_text)
//      }
// }

impl From<ParserError> for Vec<ParserError> {
    fn from(err: ParserError) -> Vec<ParserError> {
        vec![err]
    }
}

// impl<'a> error::Error for ParserError<'a> {}