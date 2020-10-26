use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct OutputExpression {

}

impl Expression for OutputExpression {
    fn accepts_tokens(tokens: &Vec<RaToken>) -> bool { false }
    fn parse(_: &Vec<RaToken>) -> std::result::Result<Self, Vec<errors::ParserError>> { todo!() }
    fn level(&self) -> u16 { todo!() }
    fn position(&self) -> (Position, Position) { todo!() }
}