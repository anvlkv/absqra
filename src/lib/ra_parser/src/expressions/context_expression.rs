use super::*;

#[derive(Serialize, Clone, Debug)]
pub struct ContextExpression {}

impl<'a> ParsedByToken<'a> for ContextExpression {
    fn new_from_token(token: RaToken<'a>) -> Result<Box<ContextExpression>, Vec<ParserError>> {
        todo!("implement new")
    }

    fn append_token(self, token: RaToken<'a>) -> Result<Box<ContextExpression>, Vec<ParserError>> {
        todo!("implement append_token")
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        todo!("implement allowed_tokens")
    }

    fn required_tokens(&self) -> Vec<TokenKind<'a>> {
        todo!("implement required_tokens")
    }
}

impl <'a> StartsWithTokens<'a> for ContextExpression {
    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        vec![
            TokenKind::OpenCurlyBrace
        ]
    }
}
