use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct ContextExpression {

}

impl Expression for ContextExpression {
    fn accepts_tokens(tokens: &[RaToken]) -> bool { false }
    fn parse(_: &[RaToken]) -> std::result::Result<Self, Vec<errors::ParserError>> { todo!("parse ContextExpression") }
    fn level(&self) -> u16 { todo!("level ContextExpression") }
    fn position(&self) -> (Position, Position) { todo!("position ContextExpression") }
}