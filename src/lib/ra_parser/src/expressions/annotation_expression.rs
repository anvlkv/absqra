use super::traits::{*};
use super::errors::ParserError;

use ra_lexer::cursor::Position;
use ra_lexer::token::{Token, TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationExpression<'a>(pub Token<'a>, pub Option<Option<Box<AnnotationExpression<'a>>>>);

impl<'a> AnnotationExpression<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError<'a>> {
        match token.kind.unwrap() {
            TokenKind::Identifier(_) => Ok(Self(token, None)),
            _ => Err(ParserError::ExpectedAGotB(token, vec![TokenKind::Identifier("")]))
        }
    }
}

impl<'a> Leveled for AnnotationExpression<'a> {
    fn get_level(&self) -> u16 {
        self.0.level
    }
}

impl<'a> Positioned for AnnotationExpression<'a> {
    fn get_position(&self) -> (Position, Position) {
        let AnnotationExpression (first_token, next_expression) = self;

        let start_position = first_token.position.0;
        let end_position = {
            if next_expression.as_ref().is_some() && next_expression.as_ref().unwrap().is_some() {
                next_expression.as_ref().unwrap().as_ref().unwrap().get_position().1
            }
            else {
                first_token.position.1
            }
        };
        (start_position, end_position)
    }
}

impl<'a> Expandable<'a, AnnotationExpression<'a>, Token<'a>> for AnnotationExpression<'a> {
    fn append_item(self, token: Token<'a>) -> Result<AnnotationExpression, ParserError<'a>> {
        let AnnotationExpression (first_token, next_expression) = self;

        if next_expression.is_some() {
            let child_expression;
            if next_expression.as_ref().unwrap().is_some() {
                child_expression = next_expression.unwrap().unwrap().append_item(token)?;
            }
            else {
                child_expression = AnnotationExpression::new(token)?
            }
            Ok(AnnotationExpression(first_token, Some(Some(Box::new(child_expression)))))
        }
        else {
            match token.kind.unwrap() {
                TokenKind::Colon => Ok(AnnotationExpression(first_token, Some(None))),
                _ => Err(ParserError::ExpectedAGotB(token, vec![TokenKind::Colon]))
            }
        }
    }
}