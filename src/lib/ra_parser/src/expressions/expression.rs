use ra_lexer::cursor::Position;
use std::rc::Rc;

use super::annotation_expression::AnnotationExpression;
use super::content_expression::ContentExpression;
use super::context_expression::ContextExpression;
use super::input_expression::InputExpression;
use super::output_expression::expression::OutputExpression;
use super::reference_expression::ReferenceExpression;


use super::*;

#[derive(Serialize, Clone, Debug)]
pub struct Expression<'a> {
    pub kind: Option<ExpressionKind<'a>>,
    pub position: (Position, Position),
    #[serde(skip_serializing)]
    buffer: Buffer<Expression<'a>>,
}

#[derive(Serialize, Clone, Debug)]
pub enum ExpressionKind<'a> {
    OutputExpression(OutputExpression<'a>),
    InputExpression(InputExpression),
    ReferenceExpression(ReferenceExpression),
    ContextExpression(ContextExpression),
    AnnotationExpression(AnnotationExpression<'a>),
    ContentExpression(ContentExpression),
}

impl<'a> ParsedByToken<'a, ExpressionKind<'a>> for ExpressionKind<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<Self>, Vec<ParserError>> {
        Ok(Box::new(match token.kind {
            k if OutputExpression::starts_with_tokens().contains(&k) => {
                Self::OutputExpression(*OutputExpression::new(token)?)
            }
            k if InputExpression::starts_with_tokens().contains(&k) => {
                Self::InputExpression(*InputExpression::new(token)?)
            }
            k if ReferenceExpression::starts_with_tokens().contains(&k) => {
                Self::ReferenceExpression(*ReferenceExpression::new(token)?)
            }
            k if ContextExpression::starts_with_tokens().contains(&k) => {
                Self::ContextExpression(*ContextExpression::new(token)?)
            }
            k if AnnotationExpression::starts_with_tokens().contains(&k) => {
                Self::AnnotationExpression(*AnnotationExpression::new(token)?)
            }
            k if ContentExpression::starts_with_tokens().contains(&k) => {
                Self::ContentExpression(*ContentExpression::new(token)?)
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

impl<'a> Buffered<'a, Expression<'a>> for Expression<'a> {
    fn new_candidates_from_token(token: &RaToken<'a>) -> Buffer<Expression<'a>> {
        let mut errors = Vec::new();
        let mut kinds = Vec::new();

        let find = |kind: &TokenKind<'a>| kind == &token.kind;
        repeat_kindly!(
            find,
            {
                match ExpressionKind::new(token.clone()) {
                    Ok(k) => kinds.push(k),
                    Err(e) => errors.extend(e),
                }
            },
            OutputExpression,
            InputExpression,
            ReferenceExpression,
            ContextExpression,
            AnnotationExpression,
            ContentExpression
        );

        kinds
            .into_iter()
            .map(|kind| Rc::new(Self {
                kind: Some(*kind),
                position: token.position,
                buffer: Vec::new(),
            }))
            .collect()
    }

    fn get_buffer(&self) -> Buffer<Expression<'a>> {
        self.buffer.clone()
    }
}

impl<'a> ParsedByToken<'a, Expression<'a>> for Expression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<Expression<'a>>, Vec<ParserError>> {
        let mut errors = Vec::new();

        let candidates = Expression::new_candidates_from_token(&token);
        let mut candidates_iter = candidates.clone().into_iter();

        match candidates_iter.next() {
            Some(mut candidate) => {
                if candidates.len() == 1 {
                    Ok(Box::new(Rc::make_mut(&mut candidate).clone()))
                }
                else {
                    Ok(Box::new(Self {
                        buffer: candidates.iter().map(|k| k.clone()).collect(),
                        position: token.position,
                        kind: None,
                    }))
                }
            }
            None => {
                errors.push(ParserError::UnexpectedToken(
                    format!("{:?}", token),
                    token.position.0,
                    Backtrace::new(),
                ));
    
                Err(errors)
            }
        }
    }

    fn append_token(self, token: RaToken<'a>) -> Result<Box<Expression<'a>>, Vec<ParserError>> {
        if self.kind.is_some() {
            assert_eq!(self.buffer.len(), 0);
            let kind = Some({
                match self.kind.unwrap() {
                    ExpressionKind::OutputExpression(expression) => {
                        ExpressionKind::OutputExpression(*expression.append_token(token)?)
                    }
                    ExpressionKind::InputExpression(expression) => {
                        ExpressionKind::InputExpression(*expression.append_token(token)?)
                    }
                    ExpressionKind::ReferenceExpression(expression) => {
                        ExpressionKind::ReferenceExpression(*expression.append_token(token)?)
                    }
                    ExpressionKind::ContextExpression(expression) => {
                        ExpressionKind::ContextExpression(*expression.append_token(token)?)
                    }
                    ExpressionKind::AnnotationExpression(expression) => {
                        ExpressionKind::AnnotationExpression(*expression.append_token(token)?)
                    }
                    ExpressionKind::ContentExpression(expression) => {
                        ExpressionKind::ContentExpression(*expression.append_token(token)?)
                    }
                }
            });

            Ok(Box::new(Self {
                kind,
                position: (self.position.0, token.position.1),
                ..self
            }))
        } else {
            assert!(self.buffer.len() > 0);

            let candidates = self.get_candidates_for_token(&token)?.clone();

            if candidates.len() == 1 {
                Ok(Box::new(Self {
                    kind: candidates.first().map(|e| e.kind.clone()).unwrap(),
                    buffer: Vec::new(),
                    ..self
                }))
            } else if candidates.len() > 1 {
                Ok(Box::new(Self {
                    buffer: candidates.iter().map(|k| k.clone()).collect(),
                    ..self
                }))
            } else {
                Err(vec![ParserError::InvalidBlock])
            }
        }
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        if self.kind.is_some() {
            self.kind.as_ref().unwrap().allowed_tokens()
        } else {
            self.buffer
                .iter()
                .map(|k| k.allowed_tokens())
                .flatten()
                .collect()
        }
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        ExpressionKind::starts_with_tokens()
    }
}
