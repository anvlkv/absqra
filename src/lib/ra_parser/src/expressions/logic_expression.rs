use serde::Serialize;
use super::{ParserError, ParsedByToken, RaToken, TokenKind};
use super::output_expression::OutputExpression;


#[derive(Serialize, Clone, Debug)]
pub enum OperationKind {

}

#[derive(Serialize, Clone, Debug)]
pub struct Operation<'a>{
    kind: OperationKind,
    token: RaToken<'a>
}

#[derive(Serialize, Clone, Debug)]
pub struct LogicExpression<'a> (RaToken<'a>, Operation<'a>, Option<Box<OutputExpression<'a>>>);


impl<'a> ParsedByToken<'a, LogicExpression<'a>> for LogicExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<LogicExpression<'a>>, Vec<ParserError>> { 
        todo!("implement new")
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<LogicExpression<'a>>, Vec<ParserError>> { 
        todo!("implement append_token")
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!("implement allowed_tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        vec![
            TokenKind::Identifier(Default::default()),
            TokenKind::Int(Default::default()),
            TokenKind::Float(Default::default())
        ]
    }

}

