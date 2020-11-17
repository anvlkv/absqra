use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct InputExpression {

}

impl Expression for InputExpression {
    fn accepts_tokens(tokens: &[RaToken]) -> bool { false }
    fn parse(_: &[RaToken]) -> std::result::Result<Self, Vec<errors::ParserError>> { todo!("parse InputExpression") }
    fn level(&self) -> u16 { todo!("level InputExpression") }
    fn position(&self) -> (Position, Position) { todo!("position InputExpression") }
}