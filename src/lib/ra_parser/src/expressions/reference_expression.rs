
use super::traits::{*};
use super::errors::ParserError;

use ra_lexer::cursor::Position;
use ra_lexer::token::{Token, TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceExpression<'a> (Token<'a>, Option<Box<ReferenceExpression<'a>>>);

impl<'a> ReferenceExpression<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError<'a>> {
        match token.kind.unwrap() {
            TokenKind::Identifier(_) => Ok(Self(token, None)),
            _ => Err(ParserError::ExpectedAGotB(token, vec![TokenKind::Identifier("")]))
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
            if next.is_some() {
                next.as_ref().unwrap().get_position().1
            }
            else {
                first_token.position.1
            }
        };

        (start_position, end_position)
    }
}

impl<'a> Expandable<'a, ReferenceExpression<'a>, Token<'a>> for ReferenceExpression<'a> {
    fn append_item(self, token: Token<'a>) -> Result<ReferenceExpression<'a>, ParserError<'a>> {
        let ReferenceExpression(first_token, next) = self;
        if next.is_none() {
            Ok(ReferenceExpression(first_token, Some(Box::new(ReferenceExpression::new(token)?))))
        }
        else {
            let updated_expression = next.unwrap().append_item(token)?;
            Ok(ReferenceExpression(first_token, Some(Box::new(updated_expression))))
        }
    }
}