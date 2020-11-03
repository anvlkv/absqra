use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum OutputExpression {
    Literal(Literal),
    Procedure(Identifier),
    Operation(Operation),
}

impl Expression for OutputExpression {
    fn accepts_tokens(tokens: &[RaToken]) -> bool {
        match tokens {
            t if t.len() == 1 => {
                Identifier::accepts_tokens(tokens) || Literal::accepts_tokens(tokens)
            },
            _ => Operation::accepts_tokens(tokens),
        }
    }

    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        match tokens {
            t if t.len() == 1 => match Identifier::parse(tokens) {
                Ok(id) => Ok(Self::Procedure(id)),
                Err(mut e) => match Literal::parse(tokens) {
                    Ok(lit) => Ok(Self::Literal(lit)),
                    Err(e2) => {
                        e.extend(e2);
                        Err(e)
                    }
                },
            },
            _ => Ok(Self::Operation(Operation::parse(tokens)?)),
        }
    }
    fn level(&self) -> u16 {
        match self {
            Self::Literal(lit) => lit.level(),
            Self::Procedure(id) => id.level(),
            Self::Operation(op) => op.level(),
        }
    }
    fn position(&self) -> (Position, Position) {
        match self {
            Self::Literal(lit) => lit.position(),
            Self::Procedure(id) => id.position(),
            Self::Operation(op) => op.position(),
        }
    }
}
