use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct ContentExpression {

}

impl Expression for ContentExpression {
    fn accepts_tokens(tokens: &[RaToken]) -> bool { false }
    fn parse(_: &[RaToken]) -> std::result::Result<Self, Vec<errors::ParserError>> { todo!() }
    fn level(&self) -> u16 { todo!() }
    fn position(&self) -> (Position, Position) { todo!() }
}