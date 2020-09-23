use super::*;
use super::math_operation::MathOperationKind;

#[derive(Serialize, Clone, Debug)]
pub struct MathExpression<'a> (
    Box<ExpressionMember<'a>>,
    Option<Box<MathOperationKind>>,
    Option<Box<ExpressionMember<'a>>>
);

impl<'a> ParsedByToken<'a, MathExpression<'a>> for MathExpression<'a> {
    fn new(token: RaToken<'a>) -> Result<Box<MathExpression<'a>>, Vec<ParserError>> { 
        todo!("implement me");
        // if Self::starts_with_tokens().contains(&token.kind) {
        //     Ok(Box::new(Self (token, None, None)))
        // }
        // else {
        //     Err(vec![ParserError::ExpectedAGotB(
        //         format!("{:?}", Self::starts_with_tokens()),
        //         format!("{:?}", token),
        //         token.position.0,
        //         Backtrace::new()
        //     )])
        // }
    }

    fn append_token(self, token: RaToken<'a>) -> Result<Box<MathExpression<'a>>, Vec<ParserError>> { 
        todo!("implement me");
        // if self.allowed_tokens().contains(&token.kind) {
        //     let MathExpression(first_token, operation, next) = self;

        //     if next.is_some() {
        //         Ok(Box::new(Self(first_token, operation, Some(next.unwrap().append_token(token)?))))
        //     }
        //     else if operation.is_some() {
        //         Ok(Box::new(Self(first_token, operation, Some(OutputExpression::new(token)?))))
        //     }
        //     else {
        //         Ok(Box::new(Self(first_token, Some(MathOperation::new(token)?), None)))
        //     }
        // }
        // else {
        //     Err(vec![
        //         ParserError::ExpectedAGotB(
        //             format!("{:?}", self.allowed_tokens()),
        //             format!("{:?}", token),
        //             token.position.0,
        //             Backtrace::new()
        //         )
        //     ])
        // }
        // if errors.len() != 0 {
        //     Err(errors);
        // }
    }

    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!("implement me");
        // let MathExpression(_, operation, next) = self;
        // if next.is_some() {
        //     next.as_ref().unwrap().allowed_tokens()
        // }
        // else if operation.is_some() {
        //     OutputExpression::starts_with_tokens()
        // }
        // else {
        //     assert!(operation.is_none());
        //     MathOperation::starts_with_tokens()
        // }
    }

    fn starts_with_tokens() -> Vec<TokenKind<'static>> { 
        vec![
            TokenKind::Identifier(Default::default()),
            TokenKind::Int(Default::default()),
            TokenKind::Float(Default::default())
        ]
    }

    fn required_tokens(&self) -> Vec<TokenKind<'a>> { 
        todo!() 
    }

}

