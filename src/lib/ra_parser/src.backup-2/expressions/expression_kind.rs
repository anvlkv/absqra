use super::*;

#[derive(Serialize, Clone, Debug)]
pub enum ExpressionKind<'a> {
    OutputExpression(OutputExpression<'a>),
    InputExpression(InputExpression),
    ReferenceExpression(ReferenceExpression),
    ContextExpression(ContextExpression),
    AnnotationExpression(AnnotationExpression<'a>),
    ContentExpression(ContentExpression),
}

impl<'a> ParsedByToken<'a> for ExpressionKind<'a> {
    fn new_from_token(token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> {
        Ok(Box::new(match token.kind {
            k if OutputExpression::starts_with_tokens().contains(&k) => {
                Self::OutputExpression(*OutputExpression::new_from_token(token)?)
            }
            k if InputExpression::starts_with_tokens().contains(&k) => {
                Self::InputExpression(*InputExpression::new_from_token(token)?)
            }
            k if ReferenceExpression::starts_with_tokens().contains(&k) => {
                Self::ReferenceExpression(*ReferenceExpression::new_from_token(token)?)
            }
            k if ContextExpression::starts_with_tokens().contains(&k) => {
                Self::ContextExpression(*ContextExpression::new_from_token(token)?)
            }
            k if AnnotationExpression::starts_with_tokens().contains(&k) => {
                Self::AnnotationExpression(*AnnotationExpression::new_from_token(token)?)
            }
            k if ContentExpression::starts_with_tokens().contains(&k) => {
                Self::ContentExpression(*ContentExpression::new_from_token(token)?)
            }
            k => {
                return Err(vec![ParserError::ExpectedAGotB(
                    format!("{:?}", Self::starts_with_tokens()),
                    format!("{:?}", k),
                    token.position.0,
                    Backtrace::new(),
                )])
            }
        }))
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<ExpressionKind>, Vec<ParserError>> {
        Ok(Box::new(match self {
            Self::OutputExpression(e) => Self::OutputExpression(*e.append_token(token)?),
            Self::InputExpression(e) => Self::InputExpression(*e.append_token(token)?),
            Self::ReferenceExpression(e) => Self::ReferenceExpression(*e.append_token(token)?),
            Self::ContextExpression(e) => Self::ContextExpression(*e.append_token(token)?),
            Self::AnnotationExpression(e) => Self::AnnotationExpression(*e.append_token(token)?),
            Self::ContentExpression(e) => Self::ContentExpression(*e.append_token(token)?),
        }))
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        match self {
            Self::OutputExpression(e) => e.allowed_tokens(),
            Self::InputExpression(e) => e.allowed_tokens(),
            Self::ReferenceExpression(e) => e.allowed_tokens(),
            Self::ContextExpression(e) => e.allowed_tokens(),
            Self::AnnotationExpression(e) => e.allowed_tokens(),
            Self::ContentExpression(e) => e.allowed_tokens(),
        }
    }

    fn required_tokens(&self) -> Vec<TokenKind<'a>> { 
        match self {
            Self::OutputExpression(e) => e.required_tokens(),
            Self::InputExpression(e) => e.required_tokens(),
            Self::ReferenceExpression(e) => e.required_tokens(),
            Self::ContextExpression(e) => e.required_tokens(),
            Self::AnnotationExpression(e) => e.required_tokens(),
            Self::ContentExpression(e) => e.required_tokens(),
        }
    }
}

impl<'a> StartsWithTokens<'a> for ExpressionKind<'a> {
    fn starts_with_tokens() -> Vec<TokenKind<'a>> {
        let mut all_tokens = Vec::new();

        all_tokens.extend(OutputExpression::starts_with_tokens());
        all_tokens.extend(InputExpression::starts_with_tokens());
        all_tokens.extend(ReferenceExpression::starts_with_tokens());
        all_tokens.extend(ContextExpression::starts_with_tokens());
        all_tokens.extend(AnnotationExpression::starts_with_tokens());
        all_tokens.extend(ContentExpression::starts_with_tokens());

        all_tokens
    }
}