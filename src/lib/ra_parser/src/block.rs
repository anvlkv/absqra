use crate::errors::ParserError;
use crate::expressions::expression::Expression;
use crate::parse_by_token::ParseByToken;
use ra_lexer::cursor::Position;
use ra_lexer::token::{RaToken, TokenKind};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Block<'a> {
    pub position: (Position, Position),
    pub level: u16,
    pub expression: Expression<'a>,
    pub children: Vec<Box<Block<'a>>>,
}

impl<'a> ParseByToken for Block<'a> {
    fn new(token: RaToken) -> Result<Self, Vec<ParserError>> {
        let expression = Expression::new(token)?;
        Ok(Self {
            expression,
            level: token.level,
            position: token.position.clone(),
            children: Vec::new(),
        })
    }

    fn append_token(self, token: RaToken) -> Result<Self, Vec<ParserError>> {
        todo!("append token");
    }

    fn allowed_tokens(&self) -> Vec<TokenKind> {
        todo!("implement allowed tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        Expression::starts_with_tokens()
    }
}
