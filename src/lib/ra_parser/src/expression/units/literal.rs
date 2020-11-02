use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum LiteralKind {
    StringLiteral(String),
    FloatLiteral(f64),
    IntLiteral(i64)
}

#[derive(Serialize, Debug, Clone)]
pub struct Literal {
    value: LiteralKind,
    token: RaToken
}

impl Expression for Literal {
    fn accepts_tokens(tokens: &[RaToken]) -> bool {
        tokens.len() == 1
            && {
                match tokens.first().unwrap().kind {
                    TokenKind::StringLiteral(_)
                    | TokenKind::FloatLiteral(_)
                    | TokenKind::IntegerLiteral(_) => true,
                    _ => false
                }
            }
    }

    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        if tokens.len() > 1 {
            let token = tokens.iter().nth(1).unwrap();
            Err(vec![
                ParserError::UnexpectedToken(
                    format!("{}", token.kind),
                    token.position.0,
                    Backtrace::new()
                )
            ])    
        }
        else {
            match tokens.first() {
                Some(token) => {
                    let token = token.clone();

                    match &token.kind {
                        TokenKind::StringLiteral(content) => Ok(Self {
                            value: LiteralKind::StringLiteral(content.to_string()),
                            token
                        }),
                        TokenKind::FloatLiteral(content) =>  Ok(Self {
                            value: LiteralKind::FloatLiteral(*content),
                            token
                        }),
                        TokenKind::IntegerLiteral(content) => Ok(Self {
                            value: LiteralKind::IntLiteral(*content),
                            token
                        }),
                        _ => Err(vec![ParserError::ExpectedAGotB(
                            format!("{:?}", vec![
                                TokenKind::StringLiteral(String::default()),
                                TokenKind::FloatLiteral(0.0),
                                TokenKind::IntegerLiteral(0),
                            ]),
                            format!("{}", token.kind),
                            token.position.0,
                            Backtrace::new(),
                        )]),
                    }
                },
                None => Err(vec![ParserError::UnexpectedEndOfInput(
                    Position::default(),
                    Backtrace::new(),
                )]),
            }
        }

    }
    
    fn level(&self) -> u16 {
        self.token.level
    }

    fn position(&self) -> (Position, Position) {
        self.token.position
    }
}