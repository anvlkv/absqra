use ra_lexer::token::{RaToken, TokenKind};
use crate::errors::ParserError;

pub (crate) trait ParsedByToken<'a> 
where Self: std::marker::Sized {
    fn new (token: RaToken<'a>) -> Result<Self, Vec<ParserError>>;
    fn append_token(self, token: RaToken<'a>) -> Result<Self, Vec<ParserError>>;
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>>;
    fn starts_with_tokens() -> Vec<TokenKind<'static>>;
}