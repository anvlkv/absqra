use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct OutputExpression {

}

impl Expression for OutputExpression {
    fn accepts_tokens(tokens: &[RaToken]) -> bool { 
        match tokens {
            t if t.len() == 1 => Identifier::accepts_tokens(tokens) || Literal::accepts_tokens(tokens),
            _ => Operation::accepts_tokens(tokens)
        }
    }

    fn parse(_: &[RaToken]) -> std::result::Result<Self, Vec<errors::ParserError>> { todo!() }
    fn level(&self) -> u16 { todo!() }
    fn position(&self) -> (Position, Position) { todo!() }
}