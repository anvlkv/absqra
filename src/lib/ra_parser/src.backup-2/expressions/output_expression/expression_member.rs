use super::*;


#[derive(Debug, Clone)]
pub enum OutputExpressionMember<'a> {
    Identifier(RaToken<'a>),
    Literal(RaToken<'a>),
    // Group(),
    // Operation(),
    Buffer(Buffer<OutputExpressionMember<'a>>)
}

impl <'a> StartsWithTokens<'a> for OutputExpressionMember<'a> {
    fn starts_with_tokens() -> Vec<TokenKind<'a>> { 
        vec![
            TokenKind::Identifier(Default::default()),
            TokenKind::IntegerLiteral(Default::default()),
            TokenKind::FloatLiteral(Default::default()),
            TokenKind::StringLiteral(Default::default()),
        ]
    }
}