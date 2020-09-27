use super::*;
use super::grouping_expression::GroupingExpression;
use super::logic_expression::LogicExpression;
use super::operation_expression::MathExpression;
use super::procedure_expression::ProcedureExpression;

use serde::ser::{Serialize, Serializer};
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub enum OutputExpressionKind<'a> {
    ProcedureExpression(ProcedureExpression<'a>),
    MathExpression(MathExpression<'a>),
    LogicExpression(LogicExpression<'a>),
    GroupingExpression(GroupingExpression<'a>),
    Buffer(Buffer<OutputExpressionKind<'a>>),
}

impl<'a> Serialize for OutputExpressionKind<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.clone() {
            Self::Buffer(v) => {
                let count: u64 = v.len().try_into().unwrap();
                serializer.serialize_u64(count)
            }
            Self::ProcedureExpression(exp) => exp.serialize(serializer),
            Self::MathExpression(exp) => exp.serialize(serializer),
            Self::LogicExpression(exp) => exp.serialize(serializer),
            Self::GroupingExpression(exp) => exp.serialize(serializer),
        }
    }
}

// impl<'a>

impl<'a> Buffered<'a, OutputExpressionKind<'a>> for OutputExpressionKind<'a> {
    fn new_candidates_from_token(token: &RaToken<'a>) -> Vec<Rc<OutputExpressionKind<'a>>> {
        let mut errors = Vec::new();
        let mut kinds = Vec::new();

        if ProcedureExpression::starts_with_tokens().contains(&token.kind) {
            match ProcedureExpression::new(token.clone()) {
                Ok(p) => kinds.push(OutputExpressionKind::ProcedureExpression(*p)),
                Err(e) => errors.extend(e)
            }
        }

        if MathExpression::starts_with_tokens().contains(&token.kind) {
            match MathExpression::new(token.clone()) {
                Ok(p) => kinds.push(OutputExpressionKind::MathExpression(*p)),
                Err(e) => errors.extend(e)
            }
        }

        if LogicExpression::starts_with_tokens().contains(&token.kind) {
            match LogicExpression::new(token.clone()) {
                Ok(p) => kinds.push(OutputExpressionKind::LogicExpression(*p)),
                Err(e) => errors.extend(e)
            }
        }

        if GroupingExpression::starts_with_tokens().contains(&token.kind) {
            match GroupingExpression::new(token.clone()) {
                Ok(p) => kinds.push(OutputExpressionKind::GroupingExpression(*p)),
                Err(e) => errors.extend(e)
            }
        }

        kinds
            .into_iter()
            .map(|kind| {
                Rc::new(kind)
            })
            .collect()
    }
    fn get_buffer(&self) -> Vec<Rc<OutputExpressionKind<'a>>> {
        match self {
            Self::Buffer(b)=> b.clone(),
            _ => vec![]
        }
    }
}

impl<'a> ParsedByToken<'a, OutputExpressionKind<'a>> for OutputExpressionKind<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<OutputExpressionKind<'a>>, Vec<ParserError>> {
        let mut candidates = Self::new_candidates_from_token(&token);

        if candidates.len() == 1 {
            Ok(Box::new(
                Rc::make_mut(candidates.first_mut().unwrap()).clone(),
            ))
        } else if candidates.len() > 1 {
            Ok(Box::new(Self::Buffer(
                candidates.iter().map(|k| k.clone()).collect(),
            )))
        } else {
            Err(vec![ParserError::InvalidBlock])
        }
    }
    
    fn append_token(
        self,
        token: RaToken<'a>,
    ) -> Result<Box<OutputExpressionKind<'a>>, Vec<ParserError>> {
        match self {
            Self::ProcedureExpression(exp) => Ok(Box::new(Self::ProcedureExpression(
                *exp.append_token(token)?,
            ))),
            Self::MathExpression(exp) => Ok(Box::new(Self::MathExpression(
                *exp.append_token(token)?,
            ))),
            Self::LogicExpression(exp) => {
                Ok(Box::new(Self::LogicExpression(*exp.append_token(token)?)))
            }
            Self::GroupingExpression(exp) => Ok(Box::new(Self::GroupingExpression(
                *exp.append_token(token)?,
            ))),
            Self::Buffer(_) => {
                let mut candidates = self.get_candidates_for_token(&token)?.clone();

                if candidates.len() == 1 {
                    Ok(Box::new(
                        Rc::make_mut(candidates.first_mut().unwrap()).clone(),
                    ))
                } else if candidates.len() > 1 {
                    Ok(Box::new(Self::Buffer(
                        candidates.iter().map(|k| k.clone()).collect(),
                    )))
                } else {
                    Err(vec![ParserError::InvalidBlock])
                }
            }
        }
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        match self {
            Self::ProcedureExpression(exp) => exp.allowed_tokens(),
            Self::MathExpression(exp) => exp.allowed_tokens(),
            Self::LogicExpression(exp) => exp.allowed_tokens(),
            Self::GroupingExpression(exp) => exp.allowed_tokens(),
            Self::Buffer(buf) => buf.iter().map(|e| e.allowed_tokens()).flatten().collect(),
        }
    }

    fn required_tokens(&self) -> Vec<TokenKind<'a>> { 
        match self {
            Self::ProcedureExpression(exp) => exp.required_tokens(),
            Self::MathExpression(exp) => exp.required_tokens(),
            Self::LogicExpression(exp) => exp.required_tokens(),
            Self::GroupingExpression(exp) => exp.required_tokens(),
            Self::Buffer(_) => self.min_required_tokens(),
        }
    }

    fn starts_with_tokens() -> Vec<TokenKind<'a>> {
        let mut kinds = vec![];
        kinds.extend(ProcedureExpression::starts_with_tokens());
        kinds.extend(MathExpression::starts_with_tokens());
        kinds.extend(LogicExpression::starts_with_tokens());
        kinds.extend(GroupingExpression::starts_with_tokens());

        kinds
    }
}