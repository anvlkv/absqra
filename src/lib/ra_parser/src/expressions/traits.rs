use super::errors::ParserError;
use ra_lexer::token::Token;
use ra_lexer::cursor::Position;

pub (crate) trait Expandable<'a, Item, ByItem> {
    fn append_item(self, item: ByItem) -> Result<Item, ParserError<'a>>;
}

pub (crate) trait ByTokenExpandableFromRoot<'a, Item> {
    fn append_item(self, token: Token<'a>, depth: Option<u16>) -> Result<Item, ParserError<'a>>;
}

pub (crate) trait Leveled {
    fn get_level(&self) -> u16;
}

pub (crate) trait Positioned {
    fn get_position(&self) -> (Position, Position);
}