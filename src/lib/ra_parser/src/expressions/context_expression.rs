use super::*;

#[derive(Serialize, Clone, Debug)]
pub struct ContextExpression {}

impl<'a> ParsedByToken<'a, ContextExpression> for ContextExpression {
    fn new(token: RaToken<'a>) -> Result<Box<ContextExpression>, Vec<ParserError>> {
        todo!("implement new")
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<ContextExpression>, Vec<ParserError>> {
        todo!("implement append_token")
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        todo!("implement allowed_tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        vec![
            TokenKind::OpenCurlyBrace
        ]
    }
}
