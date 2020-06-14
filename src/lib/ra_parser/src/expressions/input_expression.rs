
use super::traits::{*};
use super::errors::ParserError;

use ra_lexer::cursor::Position;
use ra_lexer::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct InputExpression {}

impl<'a> InputExpression {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError<'a>> {
        println!("{:?}", token);
        todo!()
    }
}

impl Leveled for InputExpression {
    fn get_level(&self) -> u16 {
        todo!()
    }
}

impl Positioned for InputExpression {
    fn get_position(&self) -> (Position, Position) {
        todo!()
    }
}

impl<'a> ByTokenExpandable<'a, InputExpression> for InputExpression {
    fn append_token(self, token: Token<'a>) -> Result<InputExpression, ParserError<'a>> {
        todo!()
    }
}