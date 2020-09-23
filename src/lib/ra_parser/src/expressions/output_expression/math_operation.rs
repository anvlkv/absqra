use super::*;

#[derive(Serialize, Clone, Debug)]
pub enum MathOperationKind {
    Add,
    Subtract,
    Divide,
    Multiply,
    Power,
}


impl<'a> ParsedByToken<'a, MathOperationKind> for MathOperationKind {
    fn new(token: RaToken<'a>) -> Result<Box<MathOperationKind>, Vec<ParserError>> { 
        if Self::starts_with_tokens().contains(&token.kind) {
            Ok(Box::new({match token.kind {
                TokenKind::Plus => Self::Add,
                TokenKind::Minus => Self::Subtract,
                TokenKind::Slash => Self::Divide,
                TokenKind::Asterisk => Self::Multiply,
                TokenKind::Power => Self::Power,
                _ => panic!("kinds not matching")
            }}))
        }
        else {
            Err(vec![
                ParserError::ExpectedAGotB(
                    format!("{:?}", Self::starts_with_tokens()),
                    format!("{:?}", token),
                    token.position.0,
                    Backtrace::new()
                )
            ])
        }
     }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<MathOperationKind>, Vec<ParserError>> { Err(vec![ParserError::InvalidExpression(token.position.0, Backtrace::new())]) }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { vec![] }
    fn required_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!() 
    }
    fn starts_with_tokens() -> Vec<TokenKind<'a>> { 
        vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Slash,
            TokenKind::Asterisk,
            TokenKind::Power
        ]
     }
}
