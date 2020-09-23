use super::*;
use super::{operation_expression, grouping_expression, logic_expression};
use logic_expression::LogicExpression;
use grouping_expression::GroupingExpression;
use operation_expression::MathExpression;

#[derive(Serialize, Clone, Debug)]
pub enum ExpressionMember<'a> {
    Identifier(RaToken<'a>),
    Literal(RaToken<'a>),
    LogicExpression(LogicExpression<'a>),
    GroupingExpression(GroupingExpression<'a>),
    MathExpression(MathExpression<'a>)
}