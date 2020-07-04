use super::errors::ParserError;
use super::traits::*;
use ra_lexer::cursor::Position;
use ra_lexer::token::{Token, TokenKind};
use serde::Serialize;
use failure::Backtrace;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ContextExpressionMemberKind {
    None,
    One,
    N,
    MSpecifier,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ContextExpressionMember {
    Target(ContextExpressionMemberKind),
    Source(ContextExpressionMemberKind),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContextExpression(ContextExpressionMember, ContextExpressionMember);

impl<'a> ContextExpression {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError> {
        // println!("{:?}", token);
        match token.kind.unwrap() {
            TokenKind::Int(1) => Ok(ContextExpression(
                ContextExpressionMember::Target(ContextExpressionMemberKind::One),
                ContextExpressionMember::Source(ContextExpressionMemberKind::None),
            )),
            TokenKind::Identifier("N") => Ok(ContextExpression(
                ContextExpressionMember::Target(ContextExpressionMemberKind::N),
                ContextExpressionMember::Source(ContextExpressionMemberKind::None),
            )),
            TokenKind::OpenCurlyBrace => Ok(ContextExpression(
                ContextExpressionMember::Target(ContextExpressionMemberKind::MSpecifier),
                ContextExpressionMember::Source(ContextExpressionMemberKind::None),
            )),
            _ => Err(ParserError::ExpectedAGotB(
                format!("{}", token),
                format!(
                    "{:?}",
                    vec![
                        TokenKind::Int(1),
                        TokenKind::Identifier("N"),
                        TokenKind::OpenCurlyBrace
                    ]
                ),
                token.position.0,
                Backtrace::new(),
            )),
        }
    }
}

impl Leveled for ContextExpression {
    fn get_level(&self) -> u16 {
        todo!()
    }
}

impl Positioned for ContextExpression {
    fn get_position(&self) -> (Position, Position) {
        todo!()
    }
}

impl<'a> Expandable<'a, ContextExpression, Token<'a>> for ContextExpression {
    fn append_item(self, token: Token<'a>) -> Result<ContextExpression, ParserError> {
        match token.kind.unwrap() {
            TokenKind::Colon => Ok(self.clone()),
            _ => Err(ParserError::ExpectedAGotB(
                format!("{}", token),
                format!("{:?}", vec![TokenKind::Colon]),
                token.position.0,
                Backtrace::new(),
            )),
        }
    }
}
