use ra_lexer::cursor::Position;

use crate::expressions::annotation_expression::AnnotationExpression;
use crate::expressions::content_expression::ContentExpression;
use crate::expressions::context_expression::ContextExpression;
use crate::expressions::input_expression::InputExpression;
use crate::expressions::output_expression::OutputExpression;
use crate::expressions::reference_expression::ReferenceExpression;

use super::{Backtrace, ParseByToken, ParserError, RaToken, TokenKind};

#[derive(Serialize, Clone, Debug)]
pub struct Expression<'a> {
    buffer: Vec<RaToken<'a>>,
    pub kind: Option<ExpressionKind<'a>>,
    pub position: (Position, Position),
}

#[derive(Serialize, Clone, Debug)]
pub enum ExpressionKind<'a> {
    OutputExpression(OutputExpression),
    InputExpression(InputExpression),
    ReferenceExpression(ReferenceExpression),
    ContextExpression(ContextExpression),
    AnnotationExpression(AnnotationExpression<'a>),
    ContentExpression(ContentExpression),
}

impl<'a> ParseByToken<'a> for Expression<'a> {
    fn new(token: RaToken<'a>) -> Result<Expression<'a>, Vec<ParserError>> {
        let mut errors = Vec::new();

        let possibly_matching = Self::starts_with_tokens()
            .into_iter()
            .filter(|kind| &token.kind == kind)
            .collect::<Vec<TokenKind>>();

        if possibly_matching.len() == 1 {
            let kind = {
                match token.kind {
                    k if OutputExpression::starts_with_tokens()
                        .into_iter()
                        .find(|kind| kind == &k)
                        .is_some() =>
                    {
                        ExpressionKind::OutputExpression(OutputExpression::new(token)?)
                    }
                    k if InputExpression::starts_with_tokens()
                        .into_iter()
                        .find(|kind| kind == &k)
                        .is_some() =>
                    {
                        ExpressionKind::InputExpression(InputExpression::new(token)?)
                    }
                    k if ReferenceExpression::starts_with_tokens()
                        .into_iter()
                        .find(|kind| kind == &k)
                        .is_some() =>
                    {
                        ExpressionKind::ReferenceExpression(ReferenceExpression::new(token)?)
                    }
                    k if ContextExpression::starts_with_tokens()
                        .into_iter()
                        .find(|kind| kind == &k)
                        .is_some() =>
                    {
                        ExpressionKind::ContextExpression(ContextExpression::new(token)?)
                    }
                    k if AnnotationExpression::starts_with_tokens()
                        .into_iter()
                        .find(|kind| kind == &k)
                        .is_some() =>
                    {
                        ExpressionKind::AnnotationExpression(AnnotationExpression::new(token)?)
                    }
                    k if ContentExpression::starts_with_tokens()
                        .into_iter()
                        .find(|kind| kind == &k)
                        .is_some() =>
                    {
                        ExpressionKind::ContentExpression(ContentExpression::new(token)?)
                    }
                    _ => {
                        errors.push(ParserError::UnexpectedToken(
                            format!("{:?}", token),
                            token.position.0,
                            Backtrace::new(),
                        ));

                        return Err(errors);
                    }
                }
            };

            Ok(Self {
                buffer: Vec::new(),
                kind: Some(kind),
                position: token.position,
            })
        } else if possibly_matching.len() > 1 {
            Ok(Self {
                buffer: vec![token],
                position: token.position,
                kind: None,
            })
        } else {
            errors.push(ParserError::ExpectedAGotB(
                format!("{:?}", Self::starts_with_tokens()),
                format!("{:?}", token.kind),
                token.position.0,
                Backtrace::new(),
            ));

            Err(errors)
        }
    }

    fn append_token(self, token: RaToken<'a>) -> Result<Self, Vec<ParserError>> {
        if self.kind.is_some() {
            assert_eq!(self.buffer.len(), 0);
            let kind = Some({
                match self.kind.unwrap() {
                    ExpressionKind::OutputExpression(expression) => ExpressionKind::OutputExpression(expression.append_token(token)?),
                    ExpressionKind::InputExpression(expression) => ExpressionKind::InputExpression(expression.append_token(token)?),
                    ExpressionKind::ReferenceExpression(expression) => ExpressionKind::ReferenceExpression(expression.append_token(token)?),
                    ExpressionKind::ContextExpression(expression) => ExpressionKind::ContextExpression(expression.append_token(token)?),
                    ExpressionKind::AnnotationExpression(expression) => ExpressionKind::AnnotationExpression(expression.append_token(token)?),
                    ExpressionKind::ContentExpression(expression) => ExpressionKind::ContentExpression(expression.append_token(token)?),
                }
            });

            Ok(Self {
                buffer: self.buffer,
                kind,
                position: (self.position.0, token.position.1)
            })
        }
        else {
            assert!(self.buffer.len() > 0);
            todo!("implement buffer parsing")
        }
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        todo!("implement allowed tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
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
