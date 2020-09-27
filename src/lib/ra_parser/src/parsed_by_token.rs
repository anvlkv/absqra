use ra_lexer::token::{RaToken, TokenKind};
use crate::errors::ParserError;

pub (crate) trait ParsedByToken<'a> : Clone 
where Self: std::marker::Sized {
    fn new_from_token (token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>>;
    fn append_token(self, token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>>;
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>>;
    fn required_tokens(&self) -> Vec<TokenKind<'a>>;
    fn starts_with_tokens() -> Vec<TokenKind<'a>>;
}
