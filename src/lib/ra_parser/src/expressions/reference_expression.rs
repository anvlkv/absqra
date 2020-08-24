use super::errors::ParserError;
use super::traits::*;
use serde::Serialize;
use failure::Backtrace;
use ra_lexer::cursor::Position;
use ra_lexer::token::{Token, TokenKind};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReferenceExpression<'a>(
    pub Token<'a>,
    pub Option<Option<Box<ReferenceExpression<'a>>>>,
);

impl<'a> ReferenceExpression<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError> {
        match token.kind.unwrap() {
            TokenKind::Identifier(_) | TokenKind::At => Ok(Self(token, None)),
            _ => Err(ParserError::ExpectedAGotB(
                format!("{}", token),
                format!("{:?}", vec![TokenKind::Identifier("")]),
                token.position.0,
                Backtrace::new()
            )),
        }
    }
}

impl<'a> Leveled for ReferenceExpression<'a> {
    fn get_level(&self) -> u16 {
        let ReferenceExpression(first_token, _) = self;
        first_token.level
    }
}

impl<'a> Positioned for ReferenceExpression<'a> {
    fn get_position(&self) -> (Position, Position) {
        let ReferenceExpression(first_token, next) = self;

        let start_position = first_token.position.0;

        let end_position = {
            if next.is_some() && next.as_ref().unwrap().is_some() {
                next.as_ref().unwrap().as_ref().unwrap().get_position().1
            } else {
                first_token.position.1
            }
        };

        (start_position, end_position)
    }
}

impl<'a> Expandable<'a, ReferenceExpression<'a>, Token<'a>> for ReferenceExpression<'a> {
    fn append_item(self, token: Token<'a>) -> Result<ReferenceExpression<'a>, ParserError> {
        let ReferenceExpression(first_token, next) = self;
        
        if next.is_none() {
            match token.kind.unwrap() {
                TokenKind::Dot => Ok(ReferenceExpression(first_token, Some(None))),
                TokenKind::Identifier(_) => {
                    if first_token.kind.unwrap() == TokenKind::At {
                        Ok(ReferenceExpression(first_token, Some(Some(Box::new(ReferenceExpression(token, None))))))
                    }
                    else {
                        Err(ParserError::ExpectedAGotB(
                            format!("{}", token),
                            format!("{:?}", vec![TokenKind::Identifier("")]),
                            token.position.0,
                            Backtrace::new()
                        ))
                    }
                }
                _ => Err(ParserError::ExpectedAGotB(
                    format!("{}", token),
                    format!("{:?}", vec![TokenKind::Dot]),
                    token.position.0,
                    Backtrace::new()
                )),
            }
        } else if next.is_some() && next.as_ref().unwrap().is_none() {
            Ok(ReferenceExpression(
                first_token,
                Some(Some(Box::new(ReferenceExpression::new(token)?))),
            ))
        } else {
            let updated_expression = next.unwrap().unwrap().append_item(token)?;
            Ok(ReferenceExpression(
                first_token,
                Some(Some(Box::new(updated_expression))),
            ))
        }
    }
}
