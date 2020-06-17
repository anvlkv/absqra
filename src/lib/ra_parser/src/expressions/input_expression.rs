
use super::traits::{*};
use super::errors::ParserError;

use ra_lexer::cursor::Position;
use ra_lexer::token::{Token, TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub enum ArgumentType<'a> {
    Named(Token<'a>),
    Ordered(u16)
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType<'a> {
    Literal(Token<'a>),
    Identifier(Token<'a>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputExpression<'a> (Option<ArgumentType<'a>>, Option<ValueType<'a>>, Option<Box<InputExpression<'a>>>);

impl<'a> InputExpression<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError<'a>> {
        match token.kind.unwrap() {
            TokenKind::Colon => Ok(Self(None, None, None)),
            _ => Err(ParserError::ExpectedAGotB(token, vec![TokenKind::Colon]))
        }
    }
}

impl<'a> Leveled for InputExpression<'a> {
    fn get_level(&self) -> u16 {
        let InputExpression(argument_type, value_type, next) = self;
        // isf argument_type.is_some() {
            // argument_type.unwrap().
        // }
        todo!()
    }
}

impl<'a> Positioned for InputExpression<'a> {
    fn get_position(&self) -> (Position, Position) {
        let InputExpression(argument_type, value_type, next) = self;
        todo!()
    }
}

impl<'a> ByTokenExpandableFromRoot<'a, InputExpression<'a>> for InputExpression<'a> {
    fn append_item(self, token: Token<'a>, depth: Option<u16>) -> Result<InputExpression<'a>, ParserError<'a>> {
        let InputExpression(argument_type, value_type, next) = self;

        let ordered_argument_depth = match depth {
            Some(d) => d,
            None => 0
        };

        if argument_type.is_none() {
            match token.kind.unwrap() {
                TokenKind::Identifier(_) => {
                    Ok(InputExpression(Some(ArgumentType::Named(token)), None, None))
                },
                TokenKind::Equals => {
                    Ok(InputExpression(Some(ArgumentType::Ordered(ordered_argument_depth)), None, None))
                },
                _ => Err(ParserError::ExpectedAGotB(token, vec![TokenKind::Identifier(""), TokenKind::Equals]))
            }
        }
        else if value_type.is_none() {
            match token.kind.unwrap() {
                TokenKind::StringLiteral(_)
                | TokenKind::Float(_)
                | TokenKind::Int(_) => {
                    Ok(InputExpression(argument_type, Some(ValueType::Literal(token)), None))
                },
                TokenKind::Identifier(_) => {
                    Ok(InputExpression(argument_type, Some(ValueType::Identifier(token)), None))
                },
                _ => Err(ParserError::ExpectedAGotB(token, vec![TokenKind::StringLiteral(""), TokenKind::Float(0.0), TokenKind::Int(0), TokenKind::Identifier("")]))
            }
        }
        else if next.is_none() {
            let next_expression = InputExpression::new(token)?;
            Ok(InputExpression(argument_type, value_type, Some(Box::new(next_expression))))
        }
        else {
            let updated_expression = next.unwrap().append_item(token, Some(ordered_argument_depth + 1))?;
            Ok(InputExpression(argument_type, value_type, Some(Box::new(updated_expression))))
        }
    }
}