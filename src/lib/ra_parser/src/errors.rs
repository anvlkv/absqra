use ra_lexer::token::{Token, TokenKind};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedIndentLevel,
    UnexpectedToken(Token),
    ExpectedAGotB(TokenKind, TokenKind),
    UnexpectedEndOfInput,
    InvalidExpression
}