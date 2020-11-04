use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum LogicOperator {
    AND(RaToken),           // &
    OR(RaToken),            // |
    NOT(RaToken),           // !
    NAND(RaToken, RaToken), // !&
    NOR(RaToken, RaToken),  // !!
    XOR(RaToken, RaToken),  // ||
    XNOR(RaToken, RaToken), // !|
}

impl Expression for LogicOperator {
    fn accepts_tokens(tokens: &[RaToken]) -> bool {
        match Self::parse(tokens) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        match tokens.len() {
            1 => match &tokens[0].kind {
                TokenKind::Ampersand => Ok(Self::AND(tokens[0].clone())),
                TokenKind::Pipe => Ok(Self::OR(tokens[0].clone())),
                TokenKind::Exclamation => Ok(Self::NOT(tokens[0].clone())),
                kind => Err(vec![ParserError::ExpectedAGotB(
                    format!(
                        "{:?}",
                        vec![
                            TokenKind::Ampersand,
                            TokenKind::Pipe,
                            TokenKind::Exclamation,
                        ]
                    ),
                    format!("{}", kind),
                    tokens[0].position.0,
                    Backtrace::new(),
                )]),
            },
            2 => match (&tokens[0].kind, &tokens[1].kind) {
                (TokenKind::Exclamation, TokenKind::Ampersand) => {
                    Ok(Self::NAND(tokens[0].clone(), tokens[1].clone()))
                },
                (TokenKind::Exclamation, TokenKind::Exclamation) => {
                    Ok(Self::NOR(tokens[0].clone(), tokens[1].clone()))
                },
                (TokenKind::Pipe, TokenKind::Pipe) => {
                    Ok(Self::XOR(tokens[0].clone(), tokens[1].clone()))
                },
                (TokenKind::Exclamation, TokenKind::Pipe) => {
                    Ok(Self::XNOR(tokens[0].clone(), tokens[1].clone()))
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
            Self::AND(token) => token.level,
            Self::OR(token) => token.level,
            Self::NOT(token) => token.level,
            Self::NOR(token, _) => token.level,
            Self::XOR(token, _) => token.level,
            Self::XNOR(token, _) => token.level,
            Self::NAND(token, _) => token.level,
        }
    }

    fn position(&self) -> (Position, Position) {
        match self {
            Self::AND(token) => token.position,
            Self::OR(token) => token.position,
            Self::NOT(token) => token.position,
            Self::NOR(token, t2) => (token.position.0, t2.position.1),
            Self::XOR(token, t2) => (token.position.0, t2.position.1),
            Self::XNOR(token, t2) => (token.position.0, t2.position.1),
            Self::NAND(token, t2) => (token.position.0, t2.position.1),
        }
    }
}
