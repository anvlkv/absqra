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
    fn accepts_tokens(tokens: &[RaToken]) -> bool {
        tokens.first().is_some()
            && tokens.iter().enumerate().all(|(i, t)| {
                if i == 0 {
                    t.kind == TokenKind::HashPound
                } else if i % 2 == 0 {
                    t.kind == TokenKind::Colon
                } else {
                    Identifier::accepts_tokens(&tokens[i..i+1])
                }
            })
    }

    fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        let mut tokens_iter = tokens.into_iter().enumerate();

        let mut parsed: Option<Self> = None;
        let mut first_token = None;
        let mut errors = Vec::new();

        while let Some((i, token)) = tokens_iter.next() {
            match i {
                0 => {
                    match &token.kind {
                        TokenKind::HashPound => first_token = Some(token),
                        kind => errors.push(ParserError::ExpectedAGotB(
                            format!("{}", TokenKind::HashPound),
                            format!("{}", kind),
                            token.position.0,
                            Backtrace::new()
                        ))
                    }
                }
                i if i % 2 == 0 => {
                    match &token.kind {
                        TokenKind::Colon => {
                            first_token = Some(token);
                        },
                        kind => {
                            errors.push(ParserError::ExpectedAGotB(
                                format!("{}", TokenKind::Colon),
                                format!("{}", kind),
                                token.position.0,
                                Backtrace::new()
                            ));
                        }
                    }
                }
                _ => {
                    match Identifier::parse(&tokens[i..i+1]) {
                        Ok(ident) => {
                            match parsed {
                                Some(mut expression) => {
                                    expression.push_next(AnnotationExpression(ident, None, first_token.unwrap().clone()));
                                    parsed = Some(expression);
                                },
                                None => {
                                    parsed = Some(AnnotationExpression(ident, None, first_token.unwrap().clone()))
                                }
                            }
                        },
                        Err(e) => errors.extend(e)
                    }

                }
            }
        }

        match (&parsed, &errors) {
            (p, e) if e.len() == 0  && p.is_some() => Ok(parsed.unwrap()),
            _ => {
                errors.push(ParserError::UnexpectedEndOfInput(
                        tokens.last().unwrap().position.0,
                        Backtrace::new(),
                    ));
                Err(errors)
            }
        }
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
