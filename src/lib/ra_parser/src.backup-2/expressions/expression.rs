use super::*;

use ra_lexer::cursor::Position;
use std::rc::Rc;
use std::cmp::Ordering;



use super::*;

#[derive(Serialize, Clone, Debug)]
pub struct Expression<'a> {
    pub kind: Option<ExpressionKind<'a>>,
    pub position: (Position, Position),
    pub level: u16,
    #[serde(skip_serializing)]
    buffer: Buffer<Expression<'a>>,
}

impl<'a> Buffered<'a> for Expression<'a> {
    fn new_candidates_from_token(token: &RaToken<'a>) -> Buffer<Expression<'a>> {
        let mut errors = Vec::new();
        let mut kinds = Vec::new();

        let find = |kind: &TokenKind<'a>| kind == &token.kind;
        repeat_kindly!(
            find,
            {
                match ExpressionKind::new_from_token(token.clone()) {
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
                level: token.level,
                buffer: Vec::new(),
            }))
            .collect()
    }

    fn get_buffer(&self) -> Buffer<Expression<'a>> {
        self.buffer.clone()
    }
}

impl<'a> ParsedByToken<'a> for Expression<'a> {
    fn new_from_token(token: RaToken<'a>) -> Result<Box<Expression<'a>>, Vec<ParserError>> {
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
                        level: token.level,
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
                Err(vec![ParserError::InvalidBlock(Backtrace::new())])
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

    fn required_tokens(&self) -> Vec<TokenKind<'a>> {
        if self.kind.is_some() {
            self.kind.as_ref().unwrap().required_tokens()
        } else {
            self.min_required_tokens()
        }
    }
}

impl <'a> StartsWithTokens<'a> for Expression<'a> {
    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        ExpressionKind::starts_with_tokens()
    }
}
