use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct ReferenceExpression {

}

impl Expression for ReferenceExpression {
    fn accepts_tokens(tokens: &[RaToken]) -> bool { false }
    fn parse(_: &[RaToken]) -> std::result::Result<Self, Vec<errors::ParserError>> { todo!("parse ReferenceExpression") }
    fn level(&self) -> u16 { todo!("level ReferenceExpression") }
    fn position(&self) -> (Position, Position) { todo!("position ReferenceExpression") }
}