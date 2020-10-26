use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct AnnotationExpression(Identifier, Option<Box<AnnotationExpression>>, RaToken);

impl AnnotationExpression {
    fn push_next(&mut self, next: Self) {
        let AnnotationExpression(_, child, _) = self;

        match child {
            Some(opt) => opt.push_next(next),
            None => self.1 = Some(Box::new(next)),
        }
    }
}

impl Expression for AnnotationExpression {
    fn accepts_tokens(tokens: &Vec<RaToken>) -> bool {
        tokens.first().is_some()
            && tokens.iter().enumerate().all(|(i, t)| {
                if i == 0 {
                    t.kind == TokenKind::HashPound
                } else if i % 2 == 0 {
                    t.kind == TokenKind::Colon
                } else {
                    t.kind == TokenKind::Identifier(String::default())
                }
            })
    }
    fn parse(tokens: &Vec<RaToken>) -> Result<Self, Vec<ParserError>> {
        let mut tokens_iter = tokens.into_iter().enumerate();

        let mut parsed: Option<Self> = None;
        let mut first_token = None;

        while let Some((i, token)) = tokens_iter.next() {
            match i {
                0 => {
                    first_token = Some(token);
                }
                i if i % 2 == 0 => {
                    match &token.kind {
                        TokenKind::Colon => {
                            first_token = Some(token);
                        },
                        kind => {
                            return Err(vec![ParserError::ExpectedAGotB(
                                format!("{}", TokenKind::Colon),
                                format!("{}", kind),
                                token.position.0,
                                Backtrace::new()
                            )])
                        }
                    }
                }
                _ => {
                    let ident = Identifier::new(token.clone());

                    match parsed {
                        Some(mut expression) => {
                            expression.push_next(AnnotationExpression(ident, None, first_token.unwrap().clone()));
                            parsed = Some(expression);
                        },
                        None => {
                            parsed = Some(AnnotationExpression(ident, None, first_token.unwrap().clone()))
                        }
                    }
                }
            }
        }

        match parsed {
            Some(expression) => Ok(expression),
            None => Err(vec![ParserError::UnexpectedEndOfInput(
                tokens.last().unwrap().position.0,
                Backtrace::new(),
            )]),
        }

        // Ok(parsed.unwrap())
    }
    fn level(&self) -> u16 {
        self.2.level
    }
    fn position(&self) -> (Position, Position) {
        let AnnotationExpression(ident, next, first_token) = self;

        (first_token.position.0, {
            match next {
                Some(child) => child.position().1,
                None => ident.token.position.1
            }
        })
    }
}
