use super::traits::{*};
use super::errors::ParserError;

use ra_lexer::cursor::Position;
use ra_lexer::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationExpression();

impl<'a> AnnotationExpression {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}

impl Leveled for AnnotationExpression {
    fn get_level(&self) -> u16 {
        todo!()
    }
}

impl Positioned for AnnotationExpression {
    fn get_position(&self) -> (Position, Position) {
        todo!()
    }
}

impl<'a> ByTokenExpandable<'a, AnnotationExpression> for AnnotationExpression {
    fn append_token(self, token: Token<'a>) -> Result<AnnotationExpression, ParserError<'a>> {
        todo!()
    }
}