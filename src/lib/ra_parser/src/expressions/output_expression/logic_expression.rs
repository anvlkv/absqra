use super::*;
use super::expression::OutputExpression;
use super::logic_operation::LogicOperation;

#[derive(Serialize, Clone, Debug)]
pub struct LogicExpression<'a> (Option<Box<OutputExpression<'a>>>, Option<LogicOperation>, Option<Box<OutputExpression<'a>>>);

impl<'a> ParsedByToken<'a, LogicExpression<'a>> for LogicExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<LogicExpression<'a>>, Vec<ParserError>> { 
        if Self::starts_with_tokens().contains(&token.kind) {
            Ok(Box::new(Self(Some(OutputExpression::new(token)?), None, None)))
        }
        else {
            Err(vec![
                ParserError::ExpectedAGotB(
                    format!("{:?}", Self::starts_with_tokens()),
                    format!("{:?}", token),
                    token.position.0,
                    Backtrace::new()
                )
            ])
        }
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<LogicExpression<'a>>, Vec<ParserError>> { 
        let LogicExpression(left, op, next) = self.clone();

        todo!(" implement me! ");
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { 
        let LogicExpression(left, op, next) = self;

        todo!(" implement me! ");
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        vec![
            TokenKind::Identifier(Default::default()),
            TokenKind::Int(Default::default()),
            TokenKind::Float(Default::default())
        ]
    }

}

