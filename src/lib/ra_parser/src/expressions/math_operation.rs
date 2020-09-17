use serde::Serialize;
use failure::Backtrace;
use ra_lexer::token::{RaToken, TokenKind};
use crate::parsed_by_token::ParsedByToken;
use super::{ParserError};

#[derive(Serialize, Clone, Debug)]
pub enum OperationKind {
    Add,
    Subtract,
    Divide,
    Multiply,
    Power,
}


impl<'a> ParsedByToken<'a, OperationKind> for OperationKind {
    fn new(token: RaToken<'a>) -> Result<Box<OperationKind>, Vec<ParserError>> { 
        if Self::starts_with_tokens().contains(&token.kind) {
            Ok(Box::new({match token.kind {
                TokenKind::Plus => Self::Add,
                TokenKind::Minus => Self::Subtract,
                TokenKind::Slash => Self::Divide,
                TokenKind::Asterisk => Self::Multiply,
                TokenKind::Power => Self::Power,
                _ => panic!("kinds not matching")
            }}))
        }
        else {
            Err(vec![
                ParserError::ExpectedAGotB(
                    format!("{:?}", Self::starts_with_tokens()),
                    format!("{:?}", token),
                    token.position.0,
                    Backtrace::new()
                )
            ])
        }
     }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<OperationKind>, Vec<ParserError>> { Err(vec![ParserError::InvalidExpression(token.position.0, Backtrace::new())]) }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { vec![] }
    fn starts_with_tokens() -> Vec<TokenKind<'a>> { 
        vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Slash,
            TokenKind::Asterisk,
            TokenKind::Power
        ]
     }
}

#[derive(Serialize, Clone, Debug)]
pub struct MathOperation<'a> {
    kind: OperationKind,
    token: RaToken<'a>
}

impl<'a> ParsedByToken<'a, MathOperation<'a>> for MathOperation<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<MathOperation<'a>>, Vec<ParserError>> { todo!() }
    fn append_token(self, _: RaToken<'a>) -> Result<Box<MathOperation<'a>>, Vec<ParserError>> { todo!() }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { todo!() }
    fn starts_with_tokens() -> Vec<TokenKind<'a>> { todo!() }
}