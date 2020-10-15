use super::*;
use serde::ser::{Serialize,Serializer};
use std::convert::TryInto;


#[derive(Debug, Clone, Serialize)]
pub struct OutputExpression<'a> (
    pub OutputExpressionMember<'a>, 
    pub Option<Box<OutputExpression<'a>>>
);

impl<'a> Serialize for OutputExpressionMember<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.clone() {
            Self::Buffer(v) => {
                let count: u64 = v.len().try_into().unwrap();
                serializer.serialize_u64(count)
            }
            Self::Identifier(token) => token.serialize(serializer),
            Self::Literal(token) => token.serialize(serializer),
            // Self::Group(exp) => exp.serialize(serializer),
            // Self::Operation(exp) => exp.serialize(serializer),
        }
    }
}

impl <'a> ParsedByToken<'a> for OutputExpression<'a> {
    fn new_from_token(token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> { todo!() }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> { todo!() }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { todo!() }
    fn required_tokens(&self) -> Vec<TokenKind<'a>> { todo!() }
}

impl <'a> StartsWithTokens<'a> for OutputExpression<'a> {
    fn starts_with_tokens() -> Vec<TokenKind<'a>> { 
        OutputExpressionMember::starts_with_tokens()
    }
}