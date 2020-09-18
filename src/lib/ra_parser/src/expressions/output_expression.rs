use super::buffered::{Buffer, Buffered};
use super::grouping_expression::GroupingExpression;
use super::logic_expression::LogicExpression;
use super::operation_expression::OperationExpression;
use super::procedure_expression::ProcedureExpression;
use super::{ParsedByToken, ParserError, RaToken, TokenKind};
use failure::Backtrace;
use serde::Serialize;
use std::rc::Rc;

#[derive(Serialize, Clone, Debug)]
pub enum OutputExpressionKind<'a> {
    ProcedureExpression(ProcedureExpression<'a>),
    OperationExpression(OperationExpression<'a>),
    LogicExpression(LogicExpression<'a>),
    GroupingExpression(GroupingExpression<'a>),
}

impl<'a> ParsedByToken<'a, OutputExpressionKind<'a>> for OutputExpressionKind<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<OutputExpressionKind<'a>>, Vec<ParserError>> {
        todo!("to implement");
    }
    fn append_token(
        self,
        token: RaToken<'a>,
    ) -> Result<Box<OutputExpressionKind<'a>>, Vec<ParserError>> {
        match self {
            Self::ProcedureExpression(exp) => Ok(Box::new(Self::ProcedureExpression(
                *exp.append_token(token)?,
            ))),
            Self::OperationExpression(exp) => Ok(Box::new(Self::OperationExpression(
                *exp.append_token(token)?,
            ))),
            Self::LogicExpression(exp) => {
                Ok(Box::new(Self::LogicExpression(*exp.append_token(token)?)))
            }
            Self::GroupingExpression(exp) => Ok(Box::new(Self::GroupingExpression(
                *exp.append_token(token)?,
            ))),
        }
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        match self {
            OutputExpressionKind::ProcedureExpression(exp) => exp.allowed_tokens(),
            OutputExpressionKind::OperationExpression(exp) => exp.allowed_tokens(),
            OutputExpressionKind::LogicExpression(exp) => exp.allowed_tokens(),
            OutputExpressionKind::GroupingExpression(exp) => exp.allowed_tokens(),
        }
    }
    fn starts_with_tokens() -> Vec<TokenKind<'a>> {
        let mut kinds = vec![];
        
        kinds.extend(ProcedureExpression::starts_with_tokens());
        kinds.extend(OperationExpression::starts_with_tokens());
        kinds.extend(LogicExpression::starts_with_tokens());
        kinds.extend(GroupingExpression::starts_with_tokens());

        kinds
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct OutputExpression<'a> {
    kind: Option<OutputExpressionKind<'a>>,
    #[serde(skip_serializing)]
    buffer: Buffer<OutputExpression<'a>>,
}

impl<'a> Buffered<'a, OutputExpression<'a>> for OutputExpression<'a> {
    fn new_candidates_from_token(token: &RaToken<'a>) -> Buffer<OutputExpression<'a>> {
        let mut errors = Vec::new();
        let mut kinds = Vec::new();

        let find = |kind: &TokenKind<'a>| kind == &token.kind;
        repeat_kindly!(
            find,
            {
                match OutputExpressionKind::new(token.clone()) {
                    Ok(k) => kinds.push(k),
                    Err(e) => errors.extend(e),
                }

            },
            OperationExpression,
            LogicExpression,
            GroupingExpression,
            ProcedureExpression
        );

        kinds
            .into_iter()
            .map(|kind| {
                Rc::new(Self {
                    kind: Some(*kind),
                    buffer: Vec::new(),
                })
            })
            .collect()
    }
    fn get_buffer(&self) -> Buffer<OutputExpression<'a>> {
        self.buffer.clone()
    }
}

impl<'a> ParsedByToken<'a, OutputExpression<'a>> for OutputExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<OutputExpression<'a>>, Vec<ParserError>> {
        let mut errors = Vec::new();

        let candidates = OutputExpression::new_candidates_from_token(&token);
        let mut candidates_iter = candidates.clone().into_iter();

        match candidates_iter.next() {
            Some(mut candidate) => {
                if candidates.len() == 1 {
                    Ok(Box::new(Rc::make_mut(&mut candidate).clone()))
                } else {
                    Ok(Box::new(Self {
                        buffer: candidates.iter().map(|k| k.clone()).collect(),
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
    fn append_token(
        self,
        token: RaToken<'a>,
    ) -> Result<Box<OutputExpression<'a>>, Vec<ParserError>> {
        if self.kind.is_some() {
            let kind = self.kind.unwrap().append_token(token)?;
            Ok(Box::new(Self{
                kind: Some(*kind),
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
            self.get_buffer()
                .iter()
                .map(|exp| exp.allowed_tokens())
                .flatten()
                .collect()
        }
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        OutputExpressionKind::starts_with_tokens()
    }
}
