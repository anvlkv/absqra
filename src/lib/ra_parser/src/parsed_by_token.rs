use ra_lexer::token::{RaToken, TokenKind};
use crate::errors::ParserError;

pub (crate) trait ParsedByToken<'a, T> : Clone 
where T: std::marker::Sized {
    fn new (token: RaToken<'a>) -> Result<Box<T>, Vec<ParserError>>;
    fn append_token(self, token: RaToken<'a>) -> Result<Box<T>, Vec<ParserError>>;
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>>;
    fn required_tokens(&self) -> Vec<TokenKind<'a>>;
    fn starts_with_tokens() -> Vec<TokenKind<'a>>;
}
