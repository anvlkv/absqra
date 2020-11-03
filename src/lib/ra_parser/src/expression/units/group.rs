use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct Group {

}

impl Expression for Group {
    fn accepts_tokens(tokens: &[RaToken]) -> bool { 
        match (tokens.first(), tokens.last()) {
            (Some(open), Some(close)) => {
                let last_token_index = tokens.len() -1;
                open.kind == TokenKind::OpenParentheses && close.kind == TokenKind::CloseParentheses && Operand::accepts_tokens(&tokens[1..last_token_index])
            }
            _ => false
        }
    }
    fn parse(_: &[RaToken]) -> Result<Self, Vec<ParserError>> { todo!() }
    fn level(&self) -> u16 { todo!() }
    fn position(&self) -> (Position, Position) { todo!() }
}