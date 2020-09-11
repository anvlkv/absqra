use serde::Serialize;
use super::{ParserError, ParsedByToken, RaToken, TokenKind};

#[derive(Serialize, Clone, Debug)]
pub struct ReferenceExpression {

}

impl<'a> ParsedByToken<'a> for ReferenceExpression {
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
            TokenKind::At,
            TokenKind::Identifier(Default::default())
        ]
     }

}

