use super::*;


#[derive(Serialize, Clone, Debug)]
pub struct ContentExpression {

}

impl<'a> ParsedByToken<'a> for ContentExpression {
    fn new_from_token(token: RaToken) -> Result<Box<ContentExpression>, Vec<ParserError>> { 
        todo!("implement new")
    }

    fn append_token(self, token: RaToken) -> Result<Box<ContentExpression>, Vec<ParserError>> { 
        todo!("implement append_token")
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!("implement allowed_tokens")
    }

    fn required_tokens(&self) -> Vec<TokenKind<'a>> {
        todo!("required_tokens");
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        vec![
            TokenKind::ContentBlock
        ]
     }
}

