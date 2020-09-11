use serde::Serialize;
use super::{ParserError, ParsedByToken, RaToken, TokenKind};

#[derive(Serialize, Clone, Debug)]
pub struct ContentExpression {

}

impl<'a> ParsedByToken<'a> for ContentExpression {
    fn new(token: RaToken) -> Result<ContentExpression, Vec<ParserError>> { 
        todo!("implement new")
    }
    fn append_token(self, token: RaToken) -> Result<ContentExpression, Vec<ParserError>> { 
        todo!("implement append_token")
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!("implement allowed_tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        vec![
            TokenKind::ContentBlock
        ]
     }
}

