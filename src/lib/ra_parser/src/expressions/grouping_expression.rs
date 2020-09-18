use serde::Serialize;
use super::{ParserError, ParsedByToken, RaToken, TokenKind};
use super::output_expression::OutputExpression;
use failure::Backtrace;

#[derive(Serialize, Clone, Debug)]
pub struct GroupingExpression<'a> (Option<Box<OutputExpression<'a>>>, Option<Box<OutputExpression<'a>>>, bool);

impl<'a> ParsedByToken<'a, GroupingExpression<'a>> for GroupingExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<GroupingExpression<'a>>, Vec<ParserError>> { 
        if Self::starts_with_tokens().contains(&token.kind) {
            Ok(Box::new(Self(None, None, false)))
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
    fn append_token(self, token: RaToken<'a>) -> Result<Box<GroupingExpression<'a>>, Vec<ParserError>> { 
        let GroupingExpression(mut left, mut right, mut closed) = self;

        if closed {
            Err(vec![ParserError::InvalidExpression(
                token.position.0,
                Backtrace::new()
            )])
        }
        else if token.kind == TokenKind::CloseParentheses {
            if right.is_some() {
                if right.clone().unwrap().allowed_tokens().contains(&TokenKind::CloseParentheses) {
                    right = Some(right.unwrap().append_token(token)?);
                    Ok(Box::new(Self(left, right, closed)))
                }
                else {
                    closed = true;
                    Ok(Box::new(Self(left, right, closed)))
                }
            }
            else if left.is_some() && left.clone().unwrap().allowed_tokens().contains(&TokenKind::CloseParentheses) {
                left = Some(left.unwrap().append_token(token)?);
                Ok(Box::new(Self(left, right, closed)))
            }
            else {
                Err(vec![ParserError::InvalidExpression(
                    token.position.0,
                    Backtrace::new()
                )])
            }
        }
        else if right.is_some() {
            right = Some(right.unwrap().append_token(token)?);

            Ok(Box::new(Self(left, right, closed)))
        }
        else if left.is_some() {
            left = Some(left.unwrap().append_token(token)?);

            Ok(Box::new(Self(left, right, closed)))
        }
        else {
            left = Some(OutputExpression::new(token)?);

            Ok(Box::new(Self(left, right, closed)))
        }
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        let GroupingExpression(left, right, closed) = self;
        
        if *closed {
            vec![]
        }
        else if right.is_some() {
            right.clone().unwrap().allowed_tokens()
        }
        else if left.is_some() {
            let mut kinds = OutputExpression::starts_with_tokens();

            kinds.extend(left.clone().unwrap().allowed_tokens());

            kinds
        }
        else {
            OutputExpression::starts_with_tokens()
        }
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        vec![
            TokenKind::OpenParentheses
        ]
    }

}

