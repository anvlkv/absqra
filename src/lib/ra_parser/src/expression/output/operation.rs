use super::*;

pub struct Operation {
    left: Option<Box<Operand>>,
    right: Option<Box<Operand>>,
    op: Operator,
}

impl Expression for Operation {
    fn accepts_tokens(tokens: &[RaToken]) -> bool {
        match tokens {
            t if Operator::accepts_tokens(&t[..=1]) => {
                Operand::accepts_tokens(&t[2..])
            },
            t if Operator::accepts_tokens(&t[..=0]) => {
                Operand::accepts_tokens(&t[1..])
            },
            t if Operand::accepts_tokens(&t[..=0]) => {
                (Operator::accepts_tokens(&t[1..2]) && Operand::accepts_tokens(&t[2..]) && t.len() == 3)
                || (Operator::accepts_tokens(&t[1..=2]) && Operand::accepts_tokens(&t[3..]) && t.len() == 4)
            },
            t if t.len() >= 3 => {
                let mut left_end_index = tokens.len() - 1 - 2;
                let mut result = Group::accepts_tokens(tokens);

                while !result {
                    result = {
                        Operand::accepts_tokens(&tokens[..left_end_index])
                        && (
                            {
                                Operator::accepts_tokens(&tokens[left_end_index..left_end_index+1])
                                && Operand::accepts_tokens(&tokens[left_end_index..left_end_index+2])
                            }
                            || {
                                Operator::accepts_tokens(&tokens[left_end_index..left_end_index+2])
                                && tokens.len() >= left_end_index + 4
                                && Operand::accepts_tokens(&tokens[left_end_index..left_end_index+3])
                            }
                        )
                    };
                    if left_end_index == 0 {
                        break;
                    }
                    else {
                        left_end_index -= 1;
                    }
                }

                result
            },
            _ => false
        }
    }

    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        // match tokens.first() {
        //     Some(token) => match &token.kind {
        //         TokenKind::Identifier(_) => Ok(Self {
        //             token: token.clone(),
        //         }),
        //         _ => Err(vec![ParserError::ExpectedAGotB(
        //             format!("{}", TokenKind::Identifier("identifier".into())),
        //             format!("{}", token.kind),
        //             token.position.0,
        //             Backtrace::new(),
        //         )]),
        //     },
        //     None => Err(vec![ParserError::UnexpectedEndOfInput(
        //         Position::default(),
        //         Backtrace::new(),
        //     )]),
        // }
        todo!()
    }
    fn level(&self) -> u16 {
        todo!()
    }

    fn position(&self) -> (Position, Position) {
        todo!()
    }
}
