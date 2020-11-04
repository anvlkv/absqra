use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum MathOperator {
    Sum(RaToken),
    Subtract(RaToken),
    Divide(RaToken),
    Reminder(RaToken),
    Multiply(RaToken),
    Power(RaToken),
    Assign(RaToken),
    AddAssign(RaToken, RaToken),
    SubtractAssign(RaToken, RaToken),
}

impl Expression for MathOperator {
    fn accepts_tokens(tokens: &[RaToken]) -> bool {
        match Self::parse(tokens) {
            Ok(_) => true,
            Err(_) => false
        }
    }
    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        match tokens.len() {
            1 => match &tokens[0].kind {
                TokenKind::Plus => Ok(Self::Sum(tokens[0].clone())),
                TokenKind::Minus => Ok(Self::Subtract(tokens[0].clone())),
                TokenKind::Slash => Ok(Self::Divide(tokens[0].clone())),
                TokenKind::Percent => Ok(Self::Reminder(tokens[0].clone())),
                TokenKind::Asterisk => Ok(Self::Multiply(tokens[0].clone())),
                TokenKind::Power => Ok(Self::Power(tokens[0].clone())),
                TokenKind::Equals => Ok(Self::Assign(tokens[0].clone())),
                kind => Err(vec![ParserError::ExpectedAGotB(
                    format!("{:?}", vec![
                        TokenKind::Plus,
                        TokenKind::Minus,
                        TokenKind::Slash,
                        TokenKind::Percent,
                        TokenKind::Asterisk,
                        TokenKind::Power,
                        TokenKind::Equals,
                    ]),
                    format!("{}", kind),
                    tokens[0].position.0,
                    Backtrace::new()
                )])
            },
            2 => match (&tokens[0].kind, &tokens[1].kind) {
                (TokenKind::Plus, TokenKind::Equals) => Ok(Self::AddAssign(tokens[0].clone(), tokens[1].clone())),
                (TokenKind::Minus, TokenKind::Equals) => Ok(Self::SubtractAssign(tokens[0].clone(), tokens[1].clone())),
                (kind1, kind2) => Err(vec![
                    ParserError::ExpectedAGotB(
                        format!("{:?}", vec![
                            TokenKind::Plus,
                            TokenKind::Minus
                        ]),
                        format!("{}", kind1),
                        tokens[0].position.0,
                        Backtrace::new()
                    ),
                    ParserError::ExpectedAGotB(
                        format!("{:?}", vec![
                            TokenKind::Equals,
                        ]),
                        format!("{}", kind2),
                        tokens[1].position.0,
                        Backtrace::new()
                    )
                ])
            },
            0 => Err(vec![ParserError::UnexpectedEndOfInput(Position::default(), Backtrace::new())]),
            _ => Err(vec![ParserError::UnexpectedToken(
                format!("{}", tokens[2].kind),
                tokens[2].position.0,
                Backtrace::new()
            )]),
        }
    }

    fn level(&self) -> u16 {
        match self {
            Self::Sum(token) => token.level,
            Self::Subtract(token) => token.level,
            Self::Divide(token) => token.level,
            Self::Reminder(token) => token.level,
            Self::Multiply(token) => token.level,
            Self::Power(token) => token.level,
            Self::Assign(token) => token.level,
            Self::AddAssign(token, _) => token.level,
            Self::SubtractAssign(token, _) => token.level,
        }
    }

    
    fn position(&self) -> (Position, Position) { 
        match self {
            Self::Sum(token) => token.position,
            Self::Subtract(token) => token.position,
            Self::Divide(token) => token.position,
            Self::Reminder(token) => token.position,
            Self::Multiply(token) => token.position,
            Self::Power(token) => token.position,
            Self::Assign(token) => token.position,

            Self::AddAssign(t1, t2) => (t1.position.0, t2.position.1),
            Self::SubtractAssign(t1, t2) => (t1.position.0, t2.position.1),
        }
     }
}
