use std::error;
use std::fmt;
use ra_lexer::token::{Token, TokenKind};
use ra_lexer::cursor::Position;
use ra_lexer::errors::{LexerError};

pub enum ParserError<'a> {
    UnexpectedIndentLevel(u16, Position),
    UnexpectedToken(Token<'a>),
    ExpectedAGotB(Token<'a>, Vec<TokenKind<'a>>),
    UnexpectedEndOfInput(Position),
    InvalidExpression(Position),
    InvalidBlock,
    ContentParsingError(LexerError)
}



impl<'a> fmt::Display for ParserError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let err_text = match self {
            ParserError::UnexpectedIndentLevel(lvl, pos) =>  format!("Unexpected indentation level: {} at {}", lvl, pos),
            ParserError::UnexpectedToken(token) =>  format!("Unexpected {:?}: {} at {}", token.kind.unwrap(), token.content, token.position.0),
            ParserError::UnexpectedEndOfInput(pos) =>  format!("Unexpected end of input at {}", pos),
            ParserError::InvalidExpression(pos) =>  format!("Invalid expression at {}", pos),
            ParserError::ExpectedAGotB(got, expected) => format!("Expected one of [{:?}], got {:?}", expected, got),
            ParserError::InvalidBlock => format!("Invalid block"),
            ParserError::ContentParsingError(err) => {
                return err.fmt(f)
            }
        };

        write!(f, "Parser error: {}", err_text)
     }
}

impl<'a> fmt::Debug for ParserError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let err_text = match self {
            ParserError::UnexpectedIndentLevel(lvl, pos) =>  format!("Unexpected indentation level: {} at {}", lvl, pos),
            ParserError::UnexpectedToken(token) =>  format!("Unexpected {:?}: {} at {}", token.kind.unwrap(), token.content, token.position.0),
            ParserError::UnexpectedEndOfInput(pos) =>  format!("Unexpected end of input at {}", pos),
            ParserError::InvalidExpression(pos) =>  format!("Invalid expression at {}", pos),
            ParserError::ExpectedAGotB(got, expected) => format!("Expected one of [{:?}], got {:?}", expected, got),
            ParserError::InvalidBlock => format!("Invalid block"),
            ParserError::ContentParsingError(err) => {
                return err.fmt(f)
            }
        };

        write!(f, "{:?}", err_text)
     }
}

impl <'a> From<ParserError<'a>> for Vec<ParserError<'a>> {
    fn from(err: ParserError<'a>) -> Vec<ParserError<'a>> {
        vec![err]
    }
}

impl<'a> error::Error for ParserError<'a> {}