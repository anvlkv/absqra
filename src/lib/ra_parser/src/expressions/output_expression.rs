use super::errors::ParserError;
use super::reference_expression::ReferenceExpression;
use super::traits::*;
use failure::Backtrace;
use ra_lexer::cursor::Position;
use ra_lexer::token::{Token, TokenKind};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Copy, Serialize)]
pub enum MathOperation {
    Sum,
    Subtract,
    Divide,
    Reminder,
    Multiply,
    Power,
    AddAssign,
    SubtractAssign,
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize)]
pub enum ComparisonOperation {
    EqCompare,
    GtCompare,
    LsCompare,
    GtEqCompare,
    LsEqCompare,
    NEqCompare,
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize)]
pub enum LogicOperation {
    AND,  // &
    OR,   // |
    NOT,  // !
    NAND, // !&
    NOR,  // !!
    XOR,  // ||
    XNOR, // !|
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize)]
pub enum OperationKind {
    LogicOperation(LogicOperation),
    MathOperation(MathOperation),
    ComparisonOperation(ComparisonOperation),
    Assign,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ExpressionMember<'a> {
    Literal(Token<'a>),
    OutputExpression(bool, Option<OutputExpression<'a>>),
    ReferenceExpression(ReferenceExpression<'a>),
    Nil,
}

impl<'a> Default for ExpressionMember<'a> {
    fn default() -> Self {
        Self::Nil
    }
}

impl<'a> ExpressionMember<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError> {
        match token.kind.unwrap() {
            TokenKind::Identifier(_) => Ok(ExpressionMember::ReferenceExpression(
                ReferenceExpression::new(token)?,
            )),
            TokenKind::Int(_) | TokenKind::Float(_) | TokenKind::StringLiteral(_) => {
                Ok(ExpressionMember::Literal(token))
            }
            TokenKind::OpenParentheses => Ok(ExpressionMember::OutputExpression(false, None)),
            _ => Err(ParserError::ExpectedAGotB(
                format!("{}", token),
                format!(
                    "{:?}",
                    vec![
                        TokenKind::Identifier(""),
                        TokenKind::Int(0),
                        TokenKind::Float(0.0),
                        TokenKind::StringLiteral(""),
                        TokenKind::OpenParentheses
                    ]
                ),
                token.position.0,
                Backtrace::new(),
            )),
        }
    }
}

impl<'a> Leveled for ExpressionMember<'a> {
    fn get_level(&self) -> u16 {
        match self {
            ExpressionMember::Nil => 0,
            ExpressionMember::Literal(token) => token.level,
            ExpressionMember::ReferenceExpression(expression) => expression.get_level(),
            ExpressionMember::OutputExpression(_, expression) => {
                expression.as_ref().unwrap().get_level()
            }
        }
    }
}

impl<'a> Expandable<'a, ExpressionMember<'a>, Token<'a>> for ExpressionMember<'a> {
    fn append_item(self, token: Token<'a>) -> Result<ExpressionMember<'a>, ParserError> {
        match self {
            ExpressionMember::OutputExpression(open, expression) => {
                if open {
                    match expression {
                        Some(e) => {
                            let updated_nested_expression = e.append_item(token)?;
                            Ok(ExpressionMember::OutputExpression(
                                true,
                                Some(updated_nested_expression),
                            ))
                        }
                        None => Ok(ExpressionMember::OutputExpression(
                            true,
                            Some(OutputExpression::new(token)?),
                        )),
                    }
                } else {
                    Err(ParserError::InvalidExpression(
                        token.position.0,
                        Backtrace::new(),
                    ))
                }
            }
            _ => Err(ParserError::InvalidExpression(
                token.position.0,
                Backtrace::new(),
            )),
        }
    }
}

impl<'a> Positioned for ExpressionMember<'a> {
    fn get_position(&self) -> (Position, Position) {
        match self {
            ExpressionMember::Nil => (Position::default(), Position::default()),
            ExpressionMember::Literal(token) => token.position,
            ExpressionMember::ReferenceExpression(expression) => expression.get_position(),
            ExpressionMember::OutputExpression(_, expression) => {
                expression.as_ref().unwrap().get_position()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct OutputExpression<'a>(
    pub Box<ExpressionMember<'a>>,
    pub Option<OperationKind>,
    pub Option<Box<ExpressionMember<'a>>>,
);

impl<'a> Leveled for OutputExpression<'a> {
    fn get_level(&self) -> u16 {
        let OutputExpression(first_member, _, _) = self;
        first_member.get_level()
    }
}

impl<'a> Expandable<'a, OutputExpression<'a>, Token<'a>> for OutputExpression<'a> {
    fn append_item(self, token: Token<'a>) -> Result<OutputExpression<'a>, ParserError> {
        let OutputExpression(first_member, op, last_member) = self;
        // TODO: can this be done without matching?

        match first_member.as_ref() {
            ExpressionMember::OutputExpression(open, expression) => {
                if *open {
                    if token.kind.unwrap() == TokenKind::CloseParentheses {
                        if expression.is_some() {
                            Ok(OutputExpression(
                                Box::new(ExpressionMember::OutputExpression(
                                    false,
                                    expression.clone(),
                                )),
                                None,
                                None,
                            ))
                        } else {
                            Err(ParserError::InvalidExpression(
                                token.position.0,
                                Backtrace::new(),
                            ))
                        }
                    } else if expression.is_some() {
                        let next_expression = expression.clone().unwrap().append_item(token)?;
                        Ok(OutputExpression(
                            Box::new(ExpressionMember::OutputExpression(
                                true,
                                Some(next_expression),
                            )),
                            None,
                            None,
                        ))
                    } else {
                        Err(ParserError::InvalidBlock)
                    }
                } else if expression.is_none() {
                    Ok(OutputExpression(
                        Box::new(ExpressionMember::OutputExpression(
                            true,
                            Some(OutputExpression::new(token)?),
                        )),
                        None,
                        None,
                    ))
                } else {
                    Err(ParserError::InvalidBlock)
                }
            }
            ExpressionMember::Literal(tok) => Ok(OutputExpression(
                Box::new(ExpressionMember::OutputExpression(
                    false,
                    Some(OutputExpression::new(tok.clone())?.append_item(token)?),
                )),
                None,
                None,
            )),
            _ => {
                if op.is_none() {
                    match Self::parse_operation_first_token(token) {
                        Some(operation) => Ok(OutputExpression(
                            first_member.clone(),
                            Some(operation),
                            None,
                        )),
                        None => {
                            println!("{:?}", first_member);
                            Err(ParserError::UnexpectedToken(
                                format!("{}", token),
                                token.position.0,
                                Backtrace::new(),
                            ))
                        }
                    }
                } else {
                    match Self::parse_operation_second_token(op.unwrap(), token) {
                        Some(operation) => Ok(OutputExpression(
                            first_member.clone(),
                            Some(operation),
                            None,
                        )),
                        None => {
                            if last_member.is_none() {
                                Ok(OutputExpression(
                                    first_member.clone(),
                                    op.clone(),
                                    Some(Box::new(ExpressionMember::new(token)?)),
                                ))
                            } else {
                                let child_expression = last_member.unwrap();
                                let child_member = child_expression.append_item(token)?;
                                Ok(OutputExpression(
                                    first_member.clone(),
                                    op.clone(),
                                    Some(Box::new(child_member)),
                                ))
                            }
                        }
                    }
                }
            }
        }

        // first_member.
    }
}

impl<'a> Positioned for OutputExpression<'a> {
    fn get_position(&self) -> (Position, Position) {
        let OutputExpression(first_member, _, last_member) = self;
        let start_position = first_member.get_position().0;
        let end_position = {
            if last_member.is_some() {
                last_member.clone().unwrap().get_position().1
            } else {
                first_member.get_position().1
            }
        };

        (start_position, end_position)
    }
}

impl<'a> OutputExpression<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError> {
        let left_member = ExpressionMember::new(token)?;

        Ok(Self(Box::new(left_member), None, None))
    }

    fn parse_operation_first_token(token: Token) -> Option<OperationKind> {
        match token.kind.unwrap() {
            TokenKind::Plus => Some(OperationKind::MathOperation(MathOperation::Sum)),
            TokenKind::Minus => Some(OperationKind::MathOperation(MathOperation::Subtract)),
            TokenKind::Asterisk => Some(OperationKind::MathOperation(MathOperation::Multiply)),
            TokenKind::Slash => Some(OperationKind::MathOperation(MathOperation::Divide)),
            TokenKind::Percent => Some(OperationKind::MathOperation(MathOperation::Reminder)),
            TokenKind::Power => Some(OperationKind::MathOperation(MathOperation::Power)),

            TokenKind::Greater => Some(OperationKind::ComparisonOperation(
                ComparisonOperation::GtCompare,
            )),
            TokenKind::Less => Some(OperationKind::ComparisonOperation(
                ComparisonOperation::LsCompare,
            )),
            TokenKind::Ampersand => Some(OperationKind::LogicOperation(LogicOperation::AND)),
            TokenKind::Pipe => Some(OperationKind::LogicOperation(LogicOperation::OR)),
            TokenKind::Exclamation => Some(OperationKind::LogicOperation(LogicOperation::NOT)),
            TokenKind::Equals => Some(OperationKind::Assign),

            _ => None,
        }
    }

    fn parse_operation_second_token(op: OperationKind, token: Token) -> Option<OperationKind> {
        match token.kind.unwrap() {
            TokenKind::Equals => match op {
                OperationKind::MathOperation(m_op) => match m_op {
                    MathOperation::Sum => {
                        Some(OperationKind::MathOperation(MathOperation::AddAssign))
                    }
                    MathOperation::Subtract => {
                        Some(OperationKind::MathOperation(MathOperation::SubtractAssign))
                    }
                    _ => None,
                },
                OperationKind::ComparisonOperation(c_op) => match c_op {
                    ComparisonOperation::GtCompare => Some(OperationKind::ComparisonOperation(
                        ComparisonOperation::GtEqCompare,
                    )),
                    ComparisonOperation::LsCompare => Some(OperationKind::ComparisonOperation(
                        ComparisonOperation::LsEqCompare,
                    )),
                    _ => None,
                },
                OperationKind::LogicOperation(l_op) => match l_op {
                    LogicOperation::NOT => Some(OperationKind::ComparisonOperation(
                        ComparisonOperation::NEqCompare,
                    )),
                    _ => None,
                },
                OperationKind::Assign => Some(OperationKind::ComparisonOperation(
                    ComparisonOperation::EqCompare,
                )),
            },
            TokenKind::Ampersand => match op {
                OperationKind::LogicOperation(l_op) => match l_op {
                    LogicOperation::NOT => {
                        Some(OperationKind::LogicOperation(LogicOperation::NAND))
                    }
                    _ => None,
                },
                _ => None,
            },
            TokenKind::Pipe => match op {
                OperationKind::LogicOperation(l_op) => match l_op {
                    LogicOperation::OR => Some(OperationKind::LogicOperation(LogicOperation::XOR)),
                    LogicOperation::NOT => {
                        Some(OperationKind::LogicOperation(LogicOperation::XNOR))
                    }
                    _ => None,
                },
                _ => None,
            },
            TokenKind::Exclamation => match op {
                OperationKind::LogicOperation(l_op) => match l_op {
                    LogicOperation::NOT => Some(OperationKind::LogicOperation(LogicOperation::NOR)),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}

impl<'a> From<ReferenceExpression<'a>> for OutputExpression<'a> {
    fn from(reference_expression: ReferenceExpression<'a>) -> OutputExpression<'a> {
        OutputExpression(
            Box::new(ExpressionMember::ReferenceExpression(reference_expression)),
            None,
            None,
        )
    }
}
