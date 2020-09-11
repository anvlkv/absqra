use super::{ParsedByToken, ParserError, RaToken, TokenKind};
use failure::Backtrace;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct AnnotationExpression<'a>(
    pub Option<RaToken<'a>>,
    pub Option<Option<Box<AnnotationExpression<'a>>>>,
);

impl<'a> ParsedByToken<'a> for AnnotationExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<AnnotationExpression<'a>, Vec<ParserError>> {
        match token.kind {
            TokenKind::HashPound => Ok(Self(None, None)),
            _ => Err(vec![ParserError::ExpectedAGotB(
                format!("{:?}", Self::starts_with_tokens()),
                format!("{:?}", token.kind),
                token.position.0,
                Backtrace::new(),
            )]),
        }
    }
    fn append_token(self, token: RaToken<'a>) -> Result<AnnotationExpression<'a>, Vec<ParserError>> {
        if self
            .allowed_tokens()
            .into_iter()
            .find(|k| &token.kind == k)
            .is_some()
        {
            if self.0.is_none() {
                assert_eq!(token.kind, TokenKind::Identifier(Default::default()));
                Ok(Self(Some(token), None))
            } else if self.1.is_none() {
                assert_eq!(token.kind, TokenKind::Colon);
                Ok(Self(self.0, Some(None)))
            } else {
                let next = self.1.unwrap();

                if next.is_none() {
                    Ok(Self(self.0, Some(Some(Box::new(Self(Some(token), None))))))
                } else {
                    match next.unwrap().append_token(token) {
                        Ok(newChild) => Ok(Self(self.0, Some(Some(Box::new(newChild))))),
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            Err(vec![ParserError::ExpectedAGotB(
                format!("{:?}", self.allowed_tokens()),
                format!("{:?}", token),
                token.position.0,
                Backtrace::new(),
            )])
        }
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        if self.0.is_none() {
            vec![TokenKind::Identifier(Default::default())]
        } else if self.1.is_none() {
            vec![TokenKind::Colon]
        } else {
            let next = self.1.as_ref().unwrap();
            if next.is_none() {
                vec![TokenKind::Identifier(Default::default())]
            } else {
                next.as_ref().unwrap().allowed_tokens()
            }
        }
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        vec![TokenKind::HashPound]
    }
}
