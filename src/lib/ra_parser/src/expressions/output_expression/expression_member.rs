use super::*;
use super::{operation_expression, grouping_expression, logic_expression};
use logic_expression::LogicExpression;
use grouping_expression::GroupingExpression;
use operation_expression::OperationExpression;

#[derive(Serialize, Clone, Debug)]
pub enum ExpressionMember<'a> {
    Identifier(RaToken<'a>),
    LogicExpression(LogicExpression<'a>),
    GroupingExpression(GroupingExpression<'a>),
    OperationExpression(OperationExpression<'a>)
}