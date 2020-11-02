use super::*;


pub enum Operator {
    Math,
    Comparison,
    Logic
}

pub enum MathOperator {
    
}

pub enum ComparisonOperator {

}

pub enum LogicOperator {

}

impl Expression for Operator {
    
fn accepts_tokens(_: &[ra_lexer::RaToken]) -> bool { todo!() }
fn parse(_: &[RaToken]) -> std::result::Result<Self, std::vec::Vec<errors::ParserError>> { todo!() }
fn level(&self) -> u16 { todo!() }
fn position(&self) -> (ra_lexer::Position, ra_lexer::Position) { todo!() }
}