use crate::errors::ParserError;
use crate::expressions::expression::Expression;
use crate::finalizable::Finalizable;
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
}

impl<'a> ParsedByToken<'a, Block<'a>> for Block<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> {
        let expression = Expression::new(token)?;
        Ok(Box::new(Self {
            kind: BlockKind::Expression(*expression),
            level: token.level,
            position: token.position.clone(),
            children: Vec::new(),
        }))
    }

    fn append_token(self, token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> {
        match self.kind {
            BlockKind::Expression(old_expression) => {
                if (token.position.0).0 == (old_expression.position.0).0 {
                    let expression = old_expression.append_token(token)?;
                    Ok(Box::new(Self {
                        kind: BlockKind::Expression(*expression),
                        level: self.level,
                        position: (self.position.0, token.position.1),
                        children: self.children,
                    }))
                } else if token.level >= self.level + 1 {
                    let mut children = self.children.clone();
                    if children.len() > 0 {
                        let child = children.last().unwrap().clone().append_token(token)?;
                        children.pop();
                        children.push(child);
                        Ok(Box::new(Self {
                            children,
                            kind: BlockKind::Expression(old_expression),
                            position: (self.position.0, token.position.1),
                            ..self
                        }))
                    } else {
                        children.push(Self::new(token)?);
                        Ok(Box::new(Self {
                            children,
                            kind: BlockKind::Expression(old_expression),
                            position: (self.position.0, token.position.1),
                            ..self
                        }))
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
                    children.push(child);
                    Ok(Box::new(Self { children, ..self }))
                } else {
                    children.push(Block::new(token)?);

                    Ok(Box::new(Self { children, ..self }))
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

impl<'a> Finalizable<Block<'a>> for Block<'a> {
    fn is_final(&self) -> bool {
        todo!()
    }
    fn is_ready_to_finalize(&self) -> bool {
        todo!()
    }
    fn finalize(self) -> Result<Box<Block<'a>>, ParserError> {
        todo!()
    }
}
