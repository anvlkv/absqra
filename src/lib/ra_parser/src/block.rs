use crate::errors::ParserError;
use crate::expressions::expression::Expression;
use crate::parsed_by_token::{ParsedByToken, StartsWithTokens};
use failure::Backtrace;
use ra_lexer::cursor::Position;
use ra_lexer::token::{RaToken, TokenKind};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum BlockKind<'a> {
    Program,
    Expression(Expression<'a>),
    Union(usize),
}

#[derive(Serialize, Clone, Debug)]
pub struct Block<'a> {
    pub position: (Position, Position),
    pub level: u16,
    pub kind: BlockKind<'a>,
    pub children: Vec<Box<Block<'a>>>,
}

impl<'a> Block<'a> {
    pub fn new_program() -> Result<Box<Self>, Vec<ParserError>> {
        Ok(Box::new(Self {
            kind: BlockKind::Program,
            level: 0,
            position: (Position::default(), Position::default()),
            children: Vec::new(),
        }))
    }

    pub fn new_union(level: u16) -> Result<Box<Self>, Vec<ParserError>> {
        Ok(Box::new(Self {
            kind: BlockKind::Union(0),
            level,
            position: (Position::default(), Position::default()),
            children: Vec::new(),
        }))
    }

    fn belongs_to_primary_expression(&self, token: &RaToken<'a>) -> bool {
        match &self.kind {
            BlockKind::Program => false,
            BlockKind::Union(_) => (self.position.0).0 == (token.position.0).0,
            BlockKind::Expression(exp) => (exp.position.0).0 == (token.position.0).0
        }
    }

    fn belongs_to_sibling_expression(&self, token: &RaToken<'a>) -> bool {
        match &self.kind {
            BlockKind::Program => false,
            BlockKind::Union(_) => {
                if self.children.last().is_some() {
                    self.children.last().unwrap().belongs_to_sibling_expression(token)
                }
                else {
                    false
                }
            },
            BlockKind::Expression(exp) => exp.level == token.level && !self.belongs_to_primary_expression(token)
        }
    }

    fn belongs_to_child_expression(&self, token: &RaToken<'a>) -> bool {
        match &self.kind {
            BlockKind::Program => true,
            BlockKind::Union(_) => {
                if self.children.last().is_some() {
                    self.children.last().unwrap().belongs_to_child_expression(token)
                }
                else {
                    false
                }
            },
            BlockKind::Expression(exp) => exp.level > token.level
        }
    }

    fn append_token_child(self, token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> {
        let mut children = self.children.clone();
        if children.len() > 0 {
            let child = children.last().unwrap().clone().append_token(token)?;
            children.pop();
            children.push(child);
            Ok(Box::new(Self {
                children,
                position: (self.position.0, token.position.1),
                ..self
            }))
        } else {
            children.push(Self::new_from_token(token)?);
            Ok(Box::new(Self {
                children,
                position: (self.position.0, token.position.1),
                ..self
            }))
        }
    }
}

impl<'a> ParsedByToken<'a> for Block<'a> {
    fn new_from_token(token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> {
        let expression = Expression::new_from_token(token)?;
        Ok(Box::new(Self {
            kind: BlockKind::Expression(*expression),
            level: token.level,
            position: token.position.clone(),
            children: Vec::new(),
        }))
    }

    fn append_token(self, token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> {
        if self.belongs_to_primary_expression(&token) {
            match self.kind {
                BlockKind::Expression(old_expression) => {
                    let new_expression = old_expression.append_token(token)?;
        
                    Ok(Box::new(Self{
                        kind: BlockKind::Expression(*new_expression),
                        position: (self.position.0, token.position.1),
                        ..self
                    }))
                },
                BlockKind::Union(_) => {
                    self.append_token_child(token)
                },
                BlockKind::Program => Err(vec![ParserError::InvalidBlock(Backtrace::new())])
            }
        }
        else if self.belongs_to_child_expression(&token) {
            self.append_token_child(token)
        }
        else {
            Err(vec![ParserError::InvalidBlock(Backtrace::new())])
        }
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        match &self.kind {
            BlockKind::Expression(expression) => expression.allowed_tokens(),
            _ => Expression::starts_with_tokens(),
        }
    }

    fn required_tokens(&self) -> Vec<TokenKind<'a>> {
        match &self.kind {
            BlockKind::Expression(expression) => expression.required_tokens(),
            _ => Expression::starts_with_tokens(),
        }
    }
}

impl<'a> StartsWithTokens<'a> for Block<'a> {
    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        Expression::starts_with_tokens()
    }
}
