use super::*;
use super::logic_expression::LogicExpression;
use super::grouping_expression::GroupingExpression;
use super::operation_expression::MathExpression;

#[derive(Serialize, Clone, Debug)]
pub enum ExpressionMember<'a> {
    Identifier(RaToken<'a>),
    Literal(RaToken<'a>),
    LogicExpression(LogicExpression<'a>),
    GroupingExpression(GroupingExpression<'a>),
    MathExpression(MathExpression<'a>)
}