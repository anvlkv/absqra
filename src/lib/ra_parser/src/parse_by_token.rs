use ra_lexer::token::{RaToken, TokenKind};
use crate::errors::ParserError;

pub (crate) trait ParseByToken 
where Self: std::marker::Sized{
    fn new (token: RaToken) -> Result<Self, Vec<ParserError>>;
    fn append_token(self, token: RaToken) -> Result<Self, Vec<ParserError>>;
    fn allowed_tokens(&self) -> Vec<TokenKind>;
    fn starts_with_tokens() -> Vec<TokenKind<'static>>;
}