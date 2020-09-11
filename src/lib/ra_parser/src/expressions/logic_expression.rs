use serde::Serialize;
use super::{ParserError, ParsedByToken, RaToken, TokenKind};

#[derive(Serialize, Clone, Debug)]
pub struct LogicExpression {

}

impl<'a> ParsedByToken<'a> for LogicExpression {
    fn new(token: RaToken<'a>) -> Result<Self, Vec<ParserError>> { 
        todo!("implement new")
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Self, Vec<ParserError>> { 
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

