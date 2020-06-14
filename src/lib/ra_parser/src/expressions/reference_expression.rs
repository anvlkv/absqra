
use super::traits::{*};
use super::errors::ParserError;

use ra_lexer::cursor::Position;
use ra_lexer::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceExpression {}

impl<'a> ReferenceExpression {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError<'a>> {
        todo!()
    }
}

impl Leveled for ReferenceExpression {
    fn get_level(&self) -> u16 {
        todo!()
    }
}

impl Positioned for ReferenceExpression {
    fn get_position(&self) -> (Position, Position) {
        todo!()
    }
}

impl<'a> ByTokenExpandable<'a, ReferenceExpression> for ReferenceExpression {
    fn append_token(self, token: Token<'a>) -> Result<ReferenceExpression, ParserError<'a>> {
        todo!()
    }
}