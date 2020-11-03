use super::*;


#[derive(Serialize, Debug, Clone)]
pub enum Operator {
    Math(MathOperator),
    Comparison(ComparisonOperator),
    Logic(LogicOperator)
}



impl Expression for Operator {
    
    fn accepts_tokens(tokens: &[RaToken]) -> bool { 
        MathOperator::accepts_tokens(tokens) // || ComparisonOperator::accepts_tokens(tokens) || LogicOperator::accepts_tokens(tokens)
 }
fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> { 
    Ok(Self::Math(MathOperator::parse(tokens)?))
 }
fn level(&self) -> u16 { 
    match self {
        Self::Math(op) => op.level(),
        _ => todo!()
    }
 }
fn position(&self) -> (Position, Position) { 
    match self {
        Self::Math(op) => op.position(),
        _ => todo!()
    }
 }
}

