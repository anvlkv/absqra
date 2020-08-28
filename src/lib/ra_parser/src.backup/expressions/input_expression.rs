use super::content::Content;
use super::errors::ParserError;
use super::output_expression::OutputExpression;
use super::reference_expression::ReferenceExpression;
use super::traits::*;
use serde::Serialize;
use failure::Backtrace;

use ra_lexer::cursor::Position;
use ra_lexer::token::{Token, TokenKind};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ArgumentType<'a> {
    Named(Token<'a>, bool),
    Ordered(u16),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ValueType<'a> {
    Literal(Token<'a>),
    Content(Content<'a>),
    OutputExpression(OutputExpression<'a>),
    ReferenceExpression(ReferenceExpression<'a>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InputExpression<'a>(
    Option<ArgumentType<'a>>,
    Option<ValueType<'a>>,
    Option<Box<InputExpression<'a>>>,
);

impl<'a> InputExpression<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError> {
        match token.kind.unwrap() {
            TokenKind::Colon => Ok(Self(None, None, None)),
            _ => Err(ParserError::ExpectedAGotB(
                format!("{}", token),
                format!("{:?}", vec![TokenKind::Colon]),
                token.position.0,
                Backtrace::new()
            )),
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
    fn append_item(
        self,
        token: Token<'a>,
        depth: Option<u16>,
    ) -> Result<InputExpression<'a>, ParserError> {
        let InputExpression(argument_type, value_type, next) = self;

        let ordered_argument_depth = match depth {
            Some(d) => d,
            None => 0,
        };

        if argument_type.is_none() {
            match token.kind.unwrap() {
                TokenKind::Identifier(_) => Ok(InputExpression(
                    Some(ArgumentType::Named(token, false)),
                    None,
                    None,
                )),
                TokenKind::Equals => Ok(InputExpression(
                    Some(ArgumentType::Ordered(ordered_argument_depth)),
                    None,
                    None,
                )),
                _ => Err(ParserError::ExpectedAGotB(
                    format!("{}", token),
                    format!("{:?}", vec![TokenKind::Identifier(""), TokenKind::Equals]),
                    token.position.0,
                    Backtrace::new()
                )),
            }
        } else if value_type.is_none() {
            match argument_type.clone().unwrap() {
                ArgumentType::Named(tok, assigned) => {
                    if !assigned {
                        return match token.kind.unwrap() {
                            TokenKind::Equals => Ok(InputExpression(
                                Some(ArgumentType::Named(tok, true)),
                                None,
                                None,
                            )),
                            _ => {

                                Ok(InputExpression(
                                    Some(ArgumentType::Ordered(ordered_argument_depth)),
                                    Some(ValueType::ReferenceExpression(ReferenceExpression::new(tok)?.append_item(token)?)),
                                    None
                                ))
                            }
                        };
                    }
                }
                _ => {}
            }

            match token.kind.unwrap() {
                TokenKind::StringLiteral(_) | TokenKind::Float(_) | TokenKind::Int(_) => Ok(
                    InputExpression(argument_type, Some(ValueType::Literal(token)), None),
                ),
                TokenKind::ContentBlock => {
                    let content = Content::new(token)?;
                    Ok(InputExpression(
                        argument_type,
                        Some(ValueType::Content(content)),
                        None,
                    ))
                }
                TokenKind::Identifier(_) => Ok(InputExpression(
                    argument_type,
                    Some(ValueType::ReferenceExpression(ReferenceExpression::new(
                        token,
                    )?)),
                    None,
                )),
                _ => {
                    match OutputExpression::new(token) {
                        Ok(expression) => {
                            Ok(InputExpression(
                                argument_type,
                                Some(ValueType::OutputExpression(expression)),
                                None
                            ))
                        },
                        Err(e) => {
                            Err(ParserError::ChainedError(
                                Box::new(e),
                                Box::new(ParserError::ExpectedAGotB(
                                    format!("{}", token),
                                    format!(
                                        "{:?}",
                                        vec![
                                            TokenKind::StringLiteral(""),
                                            TokenKind::Float(0.0),
                                            TokenKind::Int(0),
                                            TokenKind::Identifier("")
                                        ]
                                    ),
                                    token.position.0,
                                    Backtrace::new()
                                ))
                            ))               
                        }
                    }
                },
            }
        } else if next.is_none() {
            match value_type.clone().unwrap() {
                ValueType::ReferenceExpression(expression) => {
                    let next_expression = match expression.clone().append_item(token) {
                        Ok(expression) => expression,
                        Err(e) => {
                            match e {
                                ParserError::ExpectedAGotB(A, B, position, trace) => {
                                    let mut next_expression: OutputExpression = expression.into();
                                    next_expression = next_expression.append_item(token)?;
                                    return Ok(InputExpression(
                                        argument_type,
                                        Some(ValueType::OutputExpression(next_expression)),
                                        None
                                    ))
                                },
                                _ => {
                                    return Err(e);
                                }
                            }
                        }
                    };
                    Ok(InputExpression(
                        argument_type,
                        Some(ValueType::ReferenceExpression(next_expression)),
                        None
                    ))
                },
                ValueType::OutputExpression(expression) => {
                    let next_expression = expression.append_item(token)?;
                    Ok(InputExpression(
                        argument_type,
                        Some(ValueType::OutputExpression(next_expression)),
                        None
                    ))
                }
                _ => {
                    let next_expression = InputExpression::new(token)?;
                    Ok(InputExpression(
                        argument_type,
                        value_type,
                        Some(Box::new(next_expression)),
                    ))
                }
            }
        } else {
            let updated_expression = next
                .unwrap()
                .append_item(token, Some(ordered_argument_depth + 1))?;
            Ok(InputExpression(
                argument_type,
                value_type,
                Some(Box::new(updated_expression)),
            ))
        }
    }
}
