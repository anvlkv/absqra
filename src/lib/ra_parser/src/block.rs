use failure::Backtrace;
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

impl<'a> ParseByToken<'a> for Block<'a> {
    fn new(token: RaToken<'a>) -> Result<Self, Vec<ParserError>> {
        let expression = Expression::new(token)?;
        Ok(Self {
            expression,
            level: token.level,
            position: token.position.clone(),
            children: Vec::new(),
        })
    }

    fn append_token(self, token: RaToken<'a>) -> Result<Self, Vec<ParserError>> {
        if (token.position.0).0 == (self.expression.position.0).0 {
            let expression = self.expression.append_token(token)?;
            Ok(Self {
                expression,
                level: self.level,
                position: (self.position.0, token.position.1),
                children: self.children
            })
        }
        else if token.level >= self.level + 1 {
            let mut children = self.children.clone();

            if children.len() > 0 {
                let child = children.last().unwrap().clone().append_token(token)?;

            
                children.pop();
                children.push(Box::new(child));
                Ok(Self {
                    children,
                    ..self
                })
            }
            else {
                children.push(Box::new(Self::new(token)?));
                Ok(Self {
                    children,
                    ..self
                })
            }
        }
        else {
            Err(vec![ParserError::UnexpectedIndentLevel(token.level, token.position.0, Backtrace::new())])
        }
        
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        self.expression.allowed_tokens()
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        Expression::starts_with_tokens()
    }
}
