use serde::Serialize;
use super::{ParserError, ParsedByToken, RaToken, TokenKind};
use super::output_expression::OutputExpression;

#[derive(Serialize, Clone, Debug)]
pub struct GroupingExpression<'a> (Option<Box<OutputExpression<'a>>>, Option<Box<OutputExpression<'a>>>);

impl<'a> ParsedByToken<'a, GroupingExpression<'a>> for GroupingExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<GroupingExpression<'a>>, Vec<ParserError>> { 
        todo!("implement new")
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<GroupingExpression<'a>>, Vec<ParserError>> { 
        todo!("implement append_token")
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!("implement allowed_tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        vec![
            TokenKind::OpenParentheses
        ]
    }

}

