use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct AnnotationExpression {

}

impl Expression for AnnotationExpression {
    fn can_parse(tokens: &Vec<RaToken>) -> bool { 
        tokens.first().is_some() && {
            let first_token = tokens.first().unwrap();
            first_token.kind == TokenKind::HashPound
        } && {
            let mut odd = tokens.iter().enumerate().filter(|(i, _)| i != &0 && i % 2 == 0);
            let mut even = tokens.iter().enumerate().filter(|(i, _)| i % 2 != 0);
            odd.all(|(_, t)| t.kind == TokenKind::Colon) && even.all(|(_, t)| t.kind == TokenKind::Identifier(String::new()))
        }
    }
    fn parse(_: &Vec<RaToken>) -> std::result::Result<Self, Vec<errors::ParserError>> { todo!() }
    fn level(&self) -> u16 { todo!() }
    fn position(&self) -> (Position, Position) { todo!() }
}