use super::*;

pub trait Expression 
where Self: Sized
{
    fn accepts_tokens(tokens: &Vec<RaToken>) -> bool;
    fn parse(tokens: &Vec<RaToken>) -> Result<Self, Vec<ParserError>>;
    fn level(&self) -> u16;
    fn position(&self) -> (Position, Position);
}