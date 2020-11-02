use super::*;
use std::hash::Hash;

#[derive(Serialize, Debug, Clone)]
pub struct Identifier {
    pub token: RaToken,
}

impl Hash for Identifier {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        match &self.token.kind {
            TokenKind::Identifier(id) => {
                String::hash(&id, hasher);
            }
            _ => panic!("expected RaToken kind Identifier"),
        }
    }
}

impl Expression for Identifier {
    fn accepts_tokens(tokens: &[RaToken]) -> bool {
        tokens.len() == 1
            && tokens.first().unwrap().kind == TokenKind::Identifier(String::default())
    }

    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        match tokens.first() {
            Some(token) => match &token.kind {
                TokenKind::Identifier(_) => Ok(Self {
                    token: token.clone(),
                }),
                _ => Err(vec![ParserError::ExpectedAGotB(
                    format!("{}", TokenKind::Identifier("identifier".into())),
                    format!("{}", token.kind),
                    token.position.0,
                    Backtrace::new(),
                )]),
            },
            None => Err(vec![ParserError::UnexpectedEndOfInput(
                Position::default(),
                Backtrace::new(),
            )]),
        }
    }

    fn level(&self) -> u16 {
        self.token.level
    }

    fn position(&self) -> (Position, Position) {
        self.token.position
    }
}
