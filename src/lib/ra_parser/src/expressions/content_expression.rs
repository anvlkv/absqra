use super::*;


#[derive(Serialize, Clone, Debug)]
pub struct ContentExpression {

}

impl<'a> ParsedByToken<'a, ContentExpression> for ContentExpression {
    fn new(token: RaToken) -> Result<Box<ContentExpression>, Vec<ParserError>> { 
        todo!("implement new")
    }
    fn append_token(self, token: RaToken) -> Result<Box<ContentExpression>, Vec<ParserError>> { 
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

