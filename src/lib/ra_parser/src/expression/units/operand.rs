use super::*;


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
    fn parse(_: &[RaToken]) -> Result<Self, Vec<ParserError>> { todo!() }
    fn level(&self) -> u16 { todo!() }
    fn position(&self) -> (Position, Position) { todo!() }
}