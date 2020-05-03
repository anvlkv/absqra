use ra_lexer::token::{Token, TokenKind};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedIndentLevel,
    UnexpectedToken,
    ExpectedAGotB,
    UnexpectedEndOfInput,
    InvalidExpression
}