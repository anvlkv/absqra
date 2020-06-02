


use super::expression::Expression;

use ra_lexer::token::{Token};
use ra_lexer::cursor::Position;

#[derive(Clone, Debug, PartialEq)]
pub enum DeclarationKind {
    Output,
    Rule
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockKind {
    Program,
    Input,
    Output,
    Declaration(Option<DeclarationKind>),
    RuleInvocation,
    Reference,
    ContextDeclaration,
    Content,
    Union(usize)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Block<'a> {
    pub kind: Option<BlockKind>,
    pub expression: Expression<'a>,
    pub position: (Position, Position),
    pub (crate) children: Vec<Block<'a>>,
    pub level: u16
}

impl <'a> Block<'a> {
    pub fn from_token_and_kind(token: Token<'a>, kind: BlockKind) -> Self {
        Self {
            kind: Some(kind),
            position: (token.position.0, token.position.1),
            expression: Expression::from_token(token),
            ..Default::default()
        }
    }

    pub fn is_complete(&self) -> bool {
        self.kind.is_some()
        && self.expression.is_complete()
        && (!self.children.last().is_some()
        || self.children.last().unwrap().is_complete())
    }
}