use super::*;

pub trait Expression 
where Self: Sized
{
    fn accepts_tokens(tokens: &[RaToken]) -> bool;
    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>>;
    fn level(&self) -> u16;
    fn position(&self) -> (Position, Position);
}

// pub trait Finite
// where Self: Expression
// {
//     fn     
// }