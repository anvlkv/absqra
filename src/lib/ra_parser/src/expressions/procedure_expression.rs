use ra_lexer::token::{RaToken, TokenKind};
use serde::Serialize;
use failure::Backtrace;

use crate::expressions::ParsedByToken;
use crate::errors::ParserError;



#[derive(Clone, Debug, Serialize)]
pub struct ProcedureExpression<'a> (RaToken<'a>);

impl<'a> ParsedByToken<'a, ProcedureExpression<'a>>  for ProcedureExpression<'a> {
    
    fn new(token: RaToken<'a>) -> Result<Box<ProcedureExpression<'a>>, Vec<ParserError>> { 
        if Self::starts_with_tokens().contains(&token.kind) {
            assert_eq!(token.kind, TokenKind::Identifier(Default::default()));
            Ok(Box::new(Self(token)))
        } 
        else {
            Err(vec![ParserError::ExpectedAGotB(
                format!("{:?}", Self::starts_with_tokens()),
                format!("{:?}", token.kind),
                token.position.0,
                Backtrace::new()
            )])
        }
     }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<ProcedureExpression<'a>>, Vec<ParserError>> { Err(vec![ParserError::InvalidExpression(token.position.0, Backtrace::new())]) }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { vec![] }
    fn starts_with_tokens() -> Vec<TokenKind<'a>> { vec![TokenKind::Identifier(Default::default())] }
}