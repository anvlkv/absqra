use ra_lexer::cursor::Position;

use crate::expressions::content_expression::ContentExpression;
use crate::expressions::annotation_expression::AnnotationExpression;
use crate::expressions::context_expression::ContextExpression;
use crate::expressions::reference_expression::ReferenceExpression;
use crate::expressions::input_expression::InputExpression;
use crate::expressions::output_expression::OutputExpression;

use super::{ParserError, ParseByToken, TokenKind, RaToken};



#[derive(Serialize, Clone, Debug)]
pub struct Expression<'a> {
    buffer: Vec<RaToken<'a>>,
    pub kind: Option<ExpressionKind>,
    pub position: (Position, Position)
}

#[derive(Serialize, Clone, Debug)]
pub enum ExpressionKind {
    OutputExpression(OutputExpression),
    InputExpression(InputExpression),
    ReferenceExpression(ReferenceExpression),
    ContextExpression(ContextExpression),
    AnnotationExpression(AnnotationExpression),
    ContentExpression(ContentExpression),
}

impl<'a> ParseByToken for Expression<'a> {
    fn new(token: RaToken) -> Result<Self, Vec<ParserError>> {
        // let tokens = OutputExpression.allowed_tokens();
        if Self::starts_with_tokens().into_iter().find(|kind| &token.kind == kind).is_some() {

        }
        // ;
        todo!();
    }

    fn append_token(self, token: RaToken) -> Result<Self, Vec<ParserError>> {
        todo!("append token to expression");
    }

    fn allowed_tokens(&self) -> Vec<TokenKind> {
        todo!("implement allowed tokens")
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> {
        let mut all_tokens = Vec::new();

        all_tokens.extend(OutputExpression::starts_with_tokens());
        all_tokens.extend(InputExpression::starts_with_tokens());
        all_tokens.extend(ReferenceExpression::starts_with_tokens());
        all_tokens.extend(ContextExpression::starts_with_tokens());
        all_tokens.extend(AnnotationExpression::starts_with_tokens());
        all_tokens.extend(ContentExpression::starts_with_tokens());

        all_tokens
    }
}