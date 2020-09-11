use crate::errors::ParserError;
use crate::expressions::expression::Expression;
use crate::parsed_by_token::ParsedByToken;
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
    pub fn new_program() -> Result<Self, Vec<ParserError>> {
        Ok(Self {
            kind: BlockKind::Program,
            level: 0,
            position: (Position::default(), Position::default()),
            children: Vec::new(),
        })
    }

    pub fn new_union(level: u16) -> Result<Self, Vec<ParserError>> {
        Ok(Self {
            kind: BlockKind::Union(0),
            level,
            position: (Position::default(), Position::default()),
            children: Vec::new(),
        })
    }
}

impl<'a> ParsedByToken<'a> for Block<'a> {
    fn new(token: RaToken<'a>) -> Result<Self, Vec<ParserError>> {
        let expression = Expression::new(token)?;
        Ok(Self {
            kind: BlockKind::Expression(expression),
            level: token.level,
            position: token.position.clone(),
            children: Vec::new(),
        })
    }

    fn append_token(self, token: RaToken<'a>) -> Result<Self, Vec<ParserError>> {
        match self.kind {
            BlockKind::Expression(old_expression) => {
                if (token.position.0).0 == (old_expression.position.0).0 {
                    let expression = old_expression.append_token(token)?;
                    Ok(Self {
                        kind: BlockKind::Expression(expression),
                        level: self.level,
                        position: (self.position.0, token.position.1),
                        children: self.children,
                    })
                } else if token.level >= self.level + 1 {
                    let mut children = self.children.clone();
                    if children.len() > 0 {
                        let child = children.last().unwrap().clone().append_token(token)?;
                        children.pop();
                        children.push(Box::new(child));
                        Ok(Self {
                            children,
                            kind: BlockKind::Expression(old_expression),
                            ..self
                        })
                    } else {
                        children.push(Box::new(Self::new(token)?));
                        Ok(Self {
                            children,
                            kind: BlockKind::Expression(old_expression),
                            ..self
                        })
                    }
                } else {
                    Err(vec![ParserError::UnexpectedIndentLevel(
                        token.level,
                        token.position.0,
                        Backtrace::new(),
                    )])
                }
            }
            BlockKind::Program => {
                let mut children = self.children.clone();

                let last_child = children.last();
                if last_child.is_some()
                    && (last_child.unwrap().level < token.level
                        || (last_child.unwrap().level == token.level
                            && (last_child.unwrap().position.1).0 == (token.position.0).0))
                {
                    let child = last_child.unwrap().clone().append_token(token)?;

                    children.pop();
                    children.push(Box::new(child));
                    Ok(Self { children, ..self })
                } else {
                    children.push(Box::new(Block::new(token)?));

                    Ok(Self { children, ..self })
                }
            }
            BlockKind::Union(_) => todo!("append to union"),
        }
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        match &self.kind {
            BlockKind::Expression(expression) => expression.allowed_tokens(),
            _ => Expression::starts_with_tokens(),
        }
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        Expression::starts_with_tokens()
    }
}
