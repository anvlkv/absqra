use super::operation_expression::OperationExpression;
use super::grouping_expression::GroupingExpression;
use super::logic_expression::LogicExpression;
use super::{ParsedByToken, ParserError, RaToken, TokenKind};
use super::buffered::{Buffer, Buffered};
use serde::Serialize;
use std::rc::Rc;

#[derive(Serialize, Clone, Debug)]
pub enum OutputExpressionKind<'a> {
    ProcedureExpression(RaToken<'a>),
    OperationExpression(OperationExpression<'a>),
    LogicExpression(LogicExpression<'a>),
    GroupingExpression(GroupingExpression<'a>)
}

#[derive(Serialize, Clone, Debug)]
pub struct OutputExpression<'a> {
    kind: OutputExpressionKind<'a>,
    #[serde(skip_serializing)]
    buffer: Buffer<OutputExpression<'a>>
}

impl<'a> Buffered<'a, OutputExpression<'a>> for OutputExpression<'a> {
    fn new_candidates_from_token(token: &RaToken<'a>) -> Buffer<OutputExpression<'a>> {
        todo!()
    }
    fn get_buffer(&self) -> Buffer<OutputExpression<'a>> {
        self.buffer.clone()
    }
}

impl<'a> ParsedByToken<'a, OutputExpression<'a>> for OutputExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<OutputExpression<'a>>, Vec<ParserError>> {
        todo!("implement new")
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<OutputExpression<'a>>, Vec<ParserError>> {
        todo!("implement append_token")
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        todo!("implement allowed_tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        let mut token_kinds = vec![
            TokenKind::Identifier(Default::default()),
            TokenKind::Int(Default::default()),
            TokenKind::Float(Default::default()),
            TokenKind::StringLiteral(Default::default()),
        ];

        token_kinds.extend(GroupingExpression::starts_with_tokens());
        token_kinds.extend(LogicExpression::starts_with_tokens());

        token_kinds
    }
}
