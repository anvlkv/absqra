use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum ComparisonOperator {
    GtCompare(RaToken), // >
    LsCompare(RaToken), // <

    EqCompare(RaToken, RaToken), // ==
    GtEqCompare(RaToken, RaToken), // >=
    LsEqCompare(RaToken, RaToken), // <=
    NEqCompare(RaToken, RaToken), // =!
}


impl Expression for ComparisonOperator {
    fn accepts_tokens(tokens: &[RaToken]) -> bool {
        match Self::parse(tokens) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        match tokens.len() {
            1 => match &tokens[0].kind {
                TokenKind::Greater => Ok(Self::GtCompare(tokens[0].clone())),
                TokenKind::Less => Ok(Self::LsCompare(tokens[0].clone())),
                kind => Err(vec![ParserError::ExpectedAGotB(
                    format!(
                        "{:?}",
                        vec![
                            TokenKind::Greater,
                            TokenKind::Less,
                        ]
                    ),
                    format!("{}", kind),
                    tokens[0].position.0,
                    Backtrace::new(),
                )]),
            },
            2 => match (&tokens[0].kind, &tokens[1].kind) {
                (TokenKind::Equals, TokenKind::Equals) => {
                    Ok(Self::EqCompare(tokens[0].clone(), tokens[1].clone()))
                },
                (TokenKind::Greater, TokenKind::Equals) => {
                    Ok(Self::GtEqCompare(tokens[0].clone(), tokens[1].clone()))
                },
                (TokenKind::Less, TokenKind::Equals) => {
                    Ok(Self::LsEqCompare(tokens[0].clone(), tokens[1].clone()))
                },
                (TokenKind::Equals, TokenKind::Exclamation) => {
                    Ok(Self::NEqCompare(tokens[0].clone(), tokens[1].clone()))
                },
                (kind1, kind2) => Err(vec![
                    ParserError::ExpectedAGotB(
                        format!("{:?}", vec![TokenKind::Plus, TokenKind::Minus]),
                        format!("{}", kind1),
                        tokens[0].position.0,
                        Backtrace::new(),
                    ),
                    ParserError::ExpectedAGotB(
                        format!("{:?}", vec![TokenKind::Equals,]),
                        format!("{}", kind2),
                        tokens[1].position.0,
                        Backtrace::new(),
                    ),
                ]),
            },
            0 => Err(vec![ParserError::UnexpectedEndOfInput(
                Position::default(),
                Backtrace::new(),
            )]),
            _ => Err(vec![ParserError::UnexpectedToken(
                format!("{}", tokens[2].kind),
                tokens[2].position.0,
                Backtrace::new(),
            )]),
        }
    }

    fn level(&self) -> u16 {
        match self {
            Self::GtCompare(token) => token.level,
            Self::LsCompare(token) => token.level,

            Self::EqCompare(token, _) => token.level,
            Self::GtEqCompare(token, _) => token.level,
            Self::LsEqCompare(token, _) => token.level,
            Self::NEqCompare(token, _) => token.level,
        }
    }

    fn position(&self) -> (Position, Position) {
        match self {
            Self::GtCompare(token) => token.position,
            Self::LsCompare(token) => token.position,

            Self::EqCompare(token, t2) => (token.position.0, t2.position.1),
            Self::GtEqCompare(token, t2) => (token.position.0, t2.position.1),
            Self::LsEqCompare(token, t2) => (token.position.0, t2.position.1),
            Self::NEqCompare(token, t2) => (token.position.0, t2.position.1),
        }
    }
}
