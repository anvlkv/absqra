use serde::Serialize;
use super::{ParserError, ParsedByToken, RaToken, TokenKind};
use super::grouping_expression::GroupingExpression;
use super::logic_expression::LogicExpression;

#[derive(Serialize, Clone, Debug)]
pub struct OutputExpression {

}

impl<'a> ParsedByToken<'a> for OutputExpression {
    fn new(token: RaToken<'a>) -> Result<Self, Vec<ParserError>> { 
        todo!("implement new")
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Self, Vec<ParserError>> { 
        todo!("implement append_token")
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!("implement allowed_tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        let mut token_kinds = vec![
            TokenKind::Identifier(Default::default()),
            TokenKind::Int(Default::default()),
            TokenKind::Float(Default::default()),
            TokenKind::StringLiteral(Default::default()),
        ];

        token_kinds.extend(GroupingExpression::starts_with_tokens());
        token_kinds.extend(LogicExpression::starts_with_tokens());

        token_kinds
    }

}

