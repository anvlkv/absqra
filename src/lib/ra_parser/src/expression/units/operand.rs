use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum Operand {
    Identifier(Identifier),
    Literal(Literal),
    Group(Group),
    Operation(Operation),
}

impl Expression for Operand {
    
    fn accepts_tokens(tokens: &[RaToken]) -> bool { 
        Identifier::accepts_tokens(tokens)
        || Literal::accepts_tokens(tokens)
        || Group::accepts_tokens(tokens)
        || Operation::accepts_tokens(tokens)
    }
    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> { 
        match tokens {
            t if Identifier::accepts_tokens(t) => Ok(Self::Identifier(Identifier::parse(tokens)?)),
            t if Literal::accepts_tokens(t) => Ok(Self::Literal(Literal::parse(tokens)?)),
            t if Group::accepts_tokens(t) => Ok(Self::Group(Group::parse(tokens)?)),
            t if Operation::accepts_tokens(t) => Ok(Self::Operation(Operation::parse(tokens)?)),
            _ => Err(vec![
                ParserError::UnexpectedEndOfInput(Position::default(), Backtrace::new())
            ])
        }
     }
    fn level(&self) -> u16 { 
        match self {
            Self::Identifier(val) => val.level(),
            Self::Literal(val) => val.level(),
            Self::Group(val) => val.level(),
            Self::Operation(val) => val.level(),
        }
     }
    fn position(&self) -> (Position, Position) { 
        match self {
            Self::Identifier(val) => val.position(),
            Self::Literal(val) => val.position(),
            Self::Group(val) => val.position(),
            Self::Operation(val) => val.position(),
        }
    }
}