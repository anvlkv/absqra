use super::*;

#[derive(Serialize, Clone, Debug)]
pub struct InputExpression {}

impl<'a> ParsedByToken<'a> for InputExpression {
    fn new_from_token(token: RaToken<'a>) -> Result<Box<InputExpression>, Vec<ParserError>> {
        todo!("implement new")
    }
    
    fn append_token(self, token: RaToken<'a>) -> Result<Box<InputExpression>, Vec<ParserError>> {
        todo!("implement append_token")
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        todo!("implement allowed_tokens")
    }

    fn required_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!() 
    }
}

impl <'a> StartsWithTokens<'a> for InputExpression {
    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        vec![
            TokenKind::Plus,
            TokenKind::Greater
        ]
    }
}
