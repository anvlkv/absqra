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

impl<'a> ParsedByToken<'a, ExpressionMember<'a>> for ExpressionMember<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<ExpressionMember<'a>>, Vec<ParserError>> { todo!() }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<ExpressionMember<'a>>, Vec<ParserError>> { todo!() }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { todo!() }
    fn required_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!() 
    }
    fn starts_with_tokens() -> Vec<TokenKind<'a>> { todo!() }
}