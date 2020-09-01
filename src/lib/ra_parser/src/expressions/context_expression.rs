use super::{ParseByToken, ParserError, RaToken, TokenKind};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ContextExpression {}

impl ParseByToken for ContextExpression {
    fn new(token: RaToken) -> std::result::Result<Self, Vec<ParserError>> {
        todo!("implement new")
    }
    fn append_token(self, token: RaToken) -> std::result::Result<Self, Vec<ParserError>> {
        todo!("implement append_token")
    }
    fn allowed_tokens(&self) -> Vec<TokenKind> {
        todo!("implement allowed_tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        [].to_vec()
    }
}
