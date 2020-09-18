use failure::Backtrace;
use serde::Serialize;
use super::{ParserError, ParsedByToken, RaToken, TokenKind};
use super::output_expression::OutputExpression;
use super::logic_operation::LogicOperation;



#[derive(Serialize, Clone, Debug)]
pub struct LogicExpression<'a> (RaToken<'a>, Option<LogicOperation>, Option<Box<OutputExpression<'a>>>);

impl<'a> ParsedByToken<'a, LogicExpression<'a>> for LogicExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<LogicExpression<'a>>, Vec<ParserError>> { 
        if Self::starts_with_tokens().contains(&token.kind) {
            Ok(Box::new(Self(token, None, None)))
        }
        else {
            Err(vec![
                ParserError::ExpectedAGotB(
                    format!("{:?}", Self::starts_with_tokens()),
                    format!("{:?}", token),
                    token.position.0,
                    Backtrace::new()
                )
            ])
        }
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<LogicExpression<'a>>, Vec<ParserError>> { 
        let LogicExpression(first_token, op, next) = self.clone();

        if op.is_none() && next.is_none() {
            Ok(Box::new(
                Self(first_token, Some(*LogicOperation::new(token)?), None)
            ))
        }
        else if op.is_some() && op.clone().unwrap().allowed_tokens().contains(&token.kind) &&  next.is_none() {
            Ok(Box::new(
                Self(first_token, Some(*op.unwrap().append_token(token)?), None)
            ))
        }
        else if op.is_some() && op.clone().unwrap().kind.is_some() {
            if next.is_some() {
                Ok(Box::new(
                    Self(first_token, op, Some(next.unwrap().append_token(token)?))
                ))
            }
            else {
                Ok(Box::new(
                    Self(first_token, op, Some(OutputExpression::new(token)?))
                ))
            }
        }
        else {
            Err(vec![
                ParserError::ExpectedAGotB(
                    format!("{:?}", self.allowed_tokens()),
                    format!("{:?}", token),
                    token.position.0,
                    Backtrace::new()
                )
            ])
        }
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { 
        let LogicExpression(_, op, next) = self;

        if next.is_some() {
            next.as_ref().unwrap().allowed_tokens().clone()
        }
        else if op.is_some() {
            op.as_ref().unwrap().allowed_tokens().clone()
        }
        else {
            LogicOperation::starts_with_tokens()
        }
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        vec![
            TokenKind::Identifier(Default::default()),
            TokenKind::Int(Default::default()),
            TokenKind::Float(Default::default())
        ]
    }

}

