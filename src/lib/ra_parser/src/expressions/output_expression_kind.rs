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