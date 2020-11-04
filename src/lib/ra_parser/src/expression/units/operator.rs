use super::*;


#[derive(Serialize, Debug, Clone)]
pub enum Operator {
    Math(MathOperator),
    Comparison(ComparisonOperator),
    Logic(LogicOperator)
}



impl Expression for Operator {
    
    fn accepts_tokens(tokens: &[RaToken]) -> bool { 
        MathOperator::accepts_tokens(tokens) || ComparisonOperator::accepts_tokens(tokens) || LogicOperator::accepts_tokens(tokens)
 }
fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> { 
    match tokens {
        t if MathOperator::accepts_tokens(t) => Ok(Self::Math(MathOperator::parse(t)?)),
        t if ComparisonOperator::accepts_tokens(t) => Ok(Self::Comparison(ComparisonOperator::parse(t)?)),
        t if LogicOperator::accepts_tokens(t) => Ok(Self::Logic(LogicOperator::parse(t)?)),
        _ => {
            Err(vec![
                ParserError::InvalidBlock(Backtrace::new())
            ])
        }
    }
 }
fn level(&self) -> u16 { 
    match self {
        Self::Math(op) => op.level(),
        Self::Comparison(op) => op.level(),
        Self::Logic(op) => op.level()
    }
 }
fn position(&self) -> (Position, Position) { 
    match self {
        Self::Math(op) => op.position(),
        Self::Comparison(op) => op.position(),
        Self::Logic(op) => op.position(),
    }
 }
}

