use ra_lexer::cursor::Position;
use std::rc::Rc;

use crate::expressions::annotation_expression::AnnotationExpression;
use crate::expressions::content_expression::ContentExpression;
use crate::expressions::context_expression::ContextExpression;
use crate::expressions::input_expression::InputExpression;
use crate::expressions::output_expression::OutputExpression;
use crate::expressions::reference_expression::ReferenceExpression;

use super::{Backtrace, ParsedByToken, ParserError, RaToken, TokenKind};

use super::buffered::Buffered;

#[derive(Serialize, Clone, Debug)]
pub struct Expression<'a> {
    pub kind: Option<ExpressionKind<'a>>,
    pub position: (Position, Position),
    #[serde(skip_serializing)]
    buffer: Vec<Rc<Expression<'a>>>,
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

impl<'a> Buffered<'a, Expression<'a>> for Expression<'a> {
    fn new_candidates_from_token(token: &RaToken<'a>) -> Vec<Expression<'a>> {
        let mut errors = Vec::new();
        let mut kinds = Vec::new();

        macro_rules! repeat_kindly {
            ($find: ident, $create_type:ident) => {
                if $create_type::starts_with_tokens()
                    .into_iter()
                    .find($find)
                    .is_some() {
                        match $create_type::new(token.clone()) {
                            Ok(exp) => kinds.push(ExpressionKind::$create_type(*exp)),
                            Err(e) => errors.extend(e)
                        }
                    }
            };
            ($find: ident,$x: ident, $($y:ident), +) => {
                repeat_kindly!($find, $x);
                repeat_kindly!($find, $($y),+);
            };
        }

        let find = |kind: &TokenKind<'a>| kind == &token.kind;

        repeat_kindly!(
            find,
            OutputExpression,
            InputExpression,
            ReferenceExpression,
            ContextExpression,
            AnnotationExpression,
            ContentExpression
        );

        kinds
            .into_iter()
            .map(|kind| Self {
                kind: Some(kind),
                position: token.position,
                buffer: Vec::new(),
            })
            .collect()
    }

    fn get_buffer(&self) -> Vec<Rc<Expression<'a>>> {
        self.buffer.clone()
    }
}

impl<'a> ParsedByToken<'a, Expression<'a>> for Expression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<Expression<'a>>, Vec<ParserError>> {
        let mut errors = Vec::new();

        let mut candidates = Expression::new_candidates_from_token(&token);

        if candidates.len() == 1 {
            Ok(Box::new(candidates.first().unwrap().clone()))
        } else if candidates.len() == 0 {
            errors.push(ParserError::UnexpectedToken(
                format!("{:?}", token),
                token.position.0,
                Backtrace::new(),
            ));

            Err(errors)
        } else {
            Ok(Box::new(Self {
                buffer: candidates.iter().map(|k| Rc::new(k.clone())).collect(),
                position: token.position,
                kind: None,
            }))
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
            // let mut candidates: Vec<ExpressionKind> = Vec::new();
            // let mut buffer = self.buffer.clone();
            // // buffer.push(token);
            // let mut buffer_iter = buffer.iter();
            // println!("{:?}", token);
            // while let Some(mut buffered_token) = buffer_iter.next() {
            //     // println!("{:?}", buffered_token);
            //     if candidates.len() == 0 {
            //         if OutputExpression::starts_with_tokens().contains(&buffered_token.kind) {
            //             candidates.push(ExpressionKind::OutputExpression(OutputExpression::new(
            //                 buffered_token.clone()
            //             )?))
            //         }
            //         if InputExpression::starts_with_tokens().contains(&buffered_token.kind) {
            //             candidates.push(ExpressionKind::InputExpression(InputExpression::new(
            //                 buffered_token.clone()
            //             )?))
            //         }
            //         if ReferenceExpression::starts_with_tokens().contains(&buffered_token.kind) {
            //             candidates.push(ExpressionKind::ReferenceExpression(
            //                 ReferenceExpression::new(buffered_token.clone())?,
            //             ))
            //         }
            //         if ContextExpression::starts_with_tokens().contains(&buffered_token.kind) {
            //             candidates.push(ExpressionKind::ContextExpression(ContextExpression::new(
            //                 buffered_token.clone()
            //             )?))
            //         }
            //         if AnnotationExpression::starts_with_tokens().contains(&buffered_token.kind) {
            //             candidates.push(ExpressionKind::AnnotationExpression(
            //                 AnnotationExpression::new(buffered_token.clone())?,
            //             ))
            //         }
            //         if ContentExpression::starts_with_tokens().contains(&buffered_token.kind) {
            //             candidates.push(ExpressionKind::ContentExpression(ContentExpression::new(
            //                 buffered_token.clone()
            //             )?))
            //         }
            //     } else {
            //         let mut candidates_iter = candidates.iter().enumerate();
            //         let mut new_candidates: Vec<ExpressionKind> = Vec::new();
            //         while let Some((index, candidate)) = candidates_iter.next() {
            //             match candidate {
            //                 ExpressionKind::OutputExpression(temp_expression) => {
            //                     if temp_expression
            //                         .allowed_tokens()
            //                         .contains(&buffered_token.kind)
            //                     {
            //                         new_candidates.push(ExpressionKind::OutputExpression(
            //                             temp_expression.clone().append_token(buffered_token.clone())?,
            //                         ))
            //                     }
            //                 }
            //                 ExpressionKind::InputExpression(temp_expression) => {
            //                     if temp_expression
            //                         .allowed_tokens()
            //                         .contains(&buffered_token.kind)
            //                     {
            //                         new_candidates.push(ExpressionKind::InputExpression(
            //                             temp_expression.clone().append_token(buffered_token.clone())?,
            //                         ))
            //                     }
            //                 }
            //                 ExpressionKind::ReferenceExpression(temp_expression) => {
            //                     if temp_expression
            //                         .allowed_tokens()
            //                         .contains(&buffered_token.kind)
            //                     {
            //                         new_candidates.push(ExpressionKind::ReferenceExpression(
            //                             temp_expression.clone().append_token(buffered_token.clone())?,
            //                         ))
            //                     }
            //                 }
            //                 ExpressionKind::ContextExpression(temp_expression) => {
            //                     if temp_expression
            //                         .allowed_tokens()
            //                         .contains(&buffered_token.kind)
            //                     {
            //                         new_candidates.push(ExpressionKind::ContextExpression(
            //                             temp_expression.clone().append_token(buffered_token.clone())?,
            //                         ))
            //                     }
            //                 }
            //                 ExpressionKind::AnnotationExpression(temp_expression) => {
            //                     if temp_expression
            //                         .allowed_tokens()
            //                         .contains(&buffered_token.kind)
            //                     {
            //                         new_candidates.push(ExpressionKind::AnnotationExpression(
            //                             temp_expression.clone().append_token(buffered_token.clone())?,
            //                         ))
            //                     }
            //                 }
            //                 ExpressionKind::ContentExpression(temp_expression) => {
            //                     if temp_expression
            //                         .allowed_tokens()
            //                         .contains(&buffered_token.kind)
            //                     {
            //                         new_candidates.push(ExpressionKind::ContentExpression(
            //                             temp_expression.clone().append_token(buffered_token.clone())?,
            //                         ))
            //                     }
            //                 }
            //             }
            //         }
            //         candidates = new_candidates;
            //     }
            // }

            let candidates = self.get_candidates_for_token(&token)?.clone();

            if candidates.len() == 1 {
                Ok(Box::new(Self {
                    kind: candidates.first().map(|e| e.kind.clone()).unwrap(),
                    buffer: Vec::new(),
                    ..self
                }))
            } else if candidates.len() > 1 {
                Ok(Box::new(Self {
                    buffer: candidates.iter().map(|k| Rc::new(k.clone())).collect(),
                    ..self
                }))
            } else {
                Err(vec![ParserError::InvalidBlock])
            }
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
