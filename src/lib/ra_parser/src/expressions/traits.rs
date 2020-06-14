use super::errors::ParserError;
use ra_lexer::token::Token;
use ra_lexer::cursor::Position;

pub (crate) trait ByTokenExpandable<'a, Item> {
    fn append_token(self, token: Token<'a>) -> Result<Item, ParserError<'a>>;
}

pub (crate) trait Leveled {
    fn get_level(&self) -> u16;
}

pub (crate) trait Positioned {
    fn get_position(&self) -> (Position, Position);
}