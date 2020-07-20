use super::errors::ParserError;
use ra_lexer::token::{Token, TokenKind};
use ra_lexer::cursor::Position;

pub (crate) trait Expandable<'a, Item, ByItem> {
    fn append_item(self, item: ByItem) -> Result<Item, ParserError>;
    // fn expected_items(&self) -> Vec<TokenKind<'a>>;
}

pub (crate) trait ByTokenExpandableFromRoot<'a, Item> {
    fn append_item(self, token: Token<'a>, depth: Option<u16>) -> Result<Item, ParserError>;
}

pub (crate) trait Leveled {
    fn get_level(&self) -> u16;
}

pub (crate) trait Positioned {
    fn get_position(&self) -> (Position, Position);
}