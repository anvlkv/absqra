use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct AnnotationExpression {

}

impl Expression for AnnotationExpression {
    fn can_parse(tokens: &Vec<RaToken>) -> bool { 
        tokens.first().is_some() && tokens.iter().enumerate().all(|(i, t)| {
            if i == 0 {
                t.kind == TokenKind::HashPound
            }
            else if i % 2 == 0 {
                t.kind == TokenKind::Colon
            }
            else {
                t.kind == TokenKind::Identifier(String::default())
            }
        })
    }
    fn parse(_: &Vec<RaToken>) -> std::result::Result<Self, Vec<errors::ParserError>> { todo!() }
    fn level(&self) -> u16 { todo!() }
    fn position(&self) -> (Position, Position) { todo!() }
}