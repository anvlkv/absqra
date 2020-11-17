use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct Group (RaToken, Box<Operand>, RaToken);

impl Expression for Group {
    fn accepts_tokens(tokens: &[RaToken]) -> bool { 
        match (tokens.first(), tokens.last()) {
            (Some(open), Some(close)) => {
                open.kind == TokenKind::OpenParentheses && close.kind == TokenKind::CloseParentheses && Operand::accepts_tokens(&tokens[1..tokens.len() -1])
            }
            _ => false
        }
    }
    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        let last_token_index = tokens.len() - 1;
        let operand = Box::new(Operand::parse(&tokens[1..last_token_index])?);
        match (&tokens[0].kind, &tokens[last_token_index].kind) {
            (TokenKind::OpenParentheses, TokenKind::CloseParentheses) => {
                Ok(Self(tokens[0].clone(), operand, tokens[last_token_index].clone()))
            },
            _ => {
                Err(vec![
                    ParserError::InvalidExpression(tokens[0].position.0, Backtrace::new())
                ])
            }
        }
    }
    fn level(&self) -> u16 { todo!() }
    fn position(&self) -> (Position, Position) { todo!() }
}