use ra_lexer::token::{Token, TokenKind};
// use std::rc::Rc;
use super::errors::ParserError;
// use std::slice::SliceIndex;


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum OperationKind {
    None,
    Sum,
    Subtract,
    Divide,
    Reminder,
    Multiply,
    Power,
    Assign,
    AddAssign,
    SubtractAssign,
    EqCompare,
    GtCompare,
    LsCompare,
    GtEqCompare,
    LsEqCompare,
    NEqCompare,
    AND, // &
    OR, // |
    NOT, // !
    NAND, // !&
    NOR, // !!
    XOR, // ||
    XNOR // !|
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ExpressionMember{
    Expression(OutputExpression, bool),
    Identifier(Token),
    Literal(Token),
    Operation(OperationKind),
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OutputExpression {
    members: [Box<ExpressionMember>; 3]
}


impl Default for OutputExpression {
    fn default() -> Self {
        OutputExpression {
            members: [Box::new(ExpressionMember::None), Box::new(ExpressionMember::None), Box::new(ExpressionMember::None)]
        }
    }
}

impl OutputExpression {
    pub fn new() -> OutputExpression {
        OutputExpression::default()
    }

    pub fn append_token(&mut self, token: Token) -> Result<(), ParserError> {
         match token.kind {
            TokenKind::OpenParentheses => {
                self.append_expression_member(ExpressionMember::Expression(OutputExpression::new(), false))
            },
            TokenKind::CloseParentheses => {
                self.close_last_expression_member()
            }
            TokenKind::Identifier => {
                self.append_expression_member(ExpressionMember::Identifier(token))
            }
            TokenKind::Float(_) |
            TokenKind::Int(_) |
            TokenKind::StringLiteral => {
                self.append_expression_member(ExpressionMember::Literal(token))
            },
            TokenKind::Plus |
            TokenKind::Minus |
            TokenKind::AddAssign |
            TokenKind::SubtractAssign |
            TokenKind::Asterisk |
            TokenKind::Slash |
            TokenKind::Percent |
            TokenKind::Power |
            TokenKind::Equals |
            TokenKind::Greater |
            TokenKind::GreaterOrEquals |
            TokenKind::Less |
            TokenKind::LessOrEquals |
            TokenKind::NotEquals => {
                self.append_operation(token)
            },
            TokenKind::Ampersand |
            TokenKind::Pipe |
            TokenKind::Exclamation => {
                self.append_logical_operation(token)
            },
            _ => Err(ParserError::UnexpectedToken(token))
        }
    }

    fn append_expression_member(&mut self, expression_member: ExpressionMember) -> Result<(), ParserError> {
        for (i, member) in self.members.iter().enumerate() {
            match member.as_ref() {
                &ExpressionMember::None => {
                    self.members[i] = Box::new(expression_member);
                    return Ok(())
                },
                &ExpressionMember::Expression(mut expression, is_complete) => {
                    if !is_complete {
                        expression.append_expression_member(expression_member);
                        return Ok(())
                    }
                }
                _ => {}
            }
        }

        Err(ParserError::InvalidExpression)
    }

    fn close_last_expression_member(&mut self) -> Result<(), ParserError> {
        for (i, member) in self.members.iter().enumerate() {
            match member.as_mut() {
                &mut ExpressionMember::Expression(mut expression, is_complete) => {
                    if !is_complete {
                        if !expression.is_nested_expression_complete() {
                            return expression.close_last_expression_member()
                        }
                        else {
                            self.members[i] = Box::new(ExpressionMember::Expression(expression, true));
                            return Ok(())
                        }
                    }
                }
                _ => {}
            }
        }

        Err(ParserError::InvalidExpression)
    }

    fn is_nested_expression_complete(&self) -> bool {
        for (i, member) in self.members.iter().enumerate() {
            match member.as_ref() {
                &ExpressionMember::Expression(expression, is_complete) => {
                    return expression.is_nested_expression_complete() && is_complete
                },
                _ => {}
            }
        }
        true
    }

    fn append_operation(&mut self, token: Token) -> Result<(), ParserError> {
        for (i, member) in self.members.iter().enumerate() {
            match member.as_mut() {
                &mut ExpressionMember::None => {
                    self.members[i] = Box::new(ExpressionMember::Operation(match &token.kind {
                        TokenKind::Plus => OperationKind::Sum,
                        TokenKind::Minus => OperationKind::Subtract,
                        TokenKind::AddAssign => OperationKind::AddAssign,
                        TokenKind::SubtractAssign => OperationKind::SubtractAssign,
                        TokenKind::Asterisk => OperationKind::Multiply,
                        TokenKind::Slash => OperationKind::Divide,
                        TokenKind::Percent => OperationKind::Reminder,
                        TokenKind::Power => OperationKind::Power,
                        TokenKind::Equals => OperationKind::EqCompare,
                        TokenKind::Greater => OperationKind::GtCompare,
                        TokenKind::GreaterOrEquals => OperationKind::GtEqCompare,
                        TokenKind::Less => OperationKind::LsCompare,
                        TokenKind::LessOrEquals => OperationKind::LsEqCompare,
                        TokenKind::NotEquals => OperationKind::NEqCompare,
                        _ => return Err(ParserError::UnexpectedToken(token))
                    }));

                    return Ok(())
                },
                &mut ExpressionMember::Expression(mut expression, is_complete) => {
                    if !is_complete {
                        return expression.append_operation(token)
                    }
                }
                _ => {}
            }
        }

        Err(ParserError::InvalidExpression)
    }

    fn append_logical_operation(&mut self, token: Token) -> Result<(), ParserError>{
        for (i, mut member) in self.members.iter().enumerate() {
            match member.as_ref() {
                &ExpressionMember::None => {
                    self.members[i] = Box::new(ExpressionMember::Operation(match &token.kind {
                        TokenKind::Ampersand => OperationKind::AND,
                        TokenKind::Pipe => OperationKind::OR,
                        TokenKind::Exclamation => OperationKind::NOT,
                        _=> return Err(ParserError::UnexpectedToken(token))
                    }));
                    return Ok(())
                },
                &ExpressionMember::Expression(mut expression, is_complete) => {
                    if !is_complete {
                        return expression.append_logical_operation(token)
                    }
                },
                &ExpressionMember::Operation(operation) => {
                    match operation {
                        OperationKind::NOT => {
                            match &token.kind {
                                TokenKind::Ampersand => {
                                    self.members[i] = Box::new(ExpressionMember::Operation(OperationKind::NAND));
                                    return Ok(())
                                },
                                TokenKind::Exclamation => {
                                    self.members[i] = Box::new(ExpressionMember::Operation(OperationKind::NOR));
                                    return Ok(())
                                },
                                TokenKind::Pipe => {
                                    self.members[i] = Box::new(ExpressionMember::Operation(OperationKind::XNOR));
                                    return Ok(())
                                },
                                _ => {}
                            }
                        },
                        OperationKind::OR => {
                            match &token.kind {
                                &TokenKind::Pipe => {
                                    self.members[i] = Box::new(ExpressionMember::Operation(OperationKind::XOR));
                                    return Ok(())
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
                _=> {}
            }
        }
        Err(ParserError::InvalidExpression)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputExpressionKind {
    OrderedAssignment(usize),
    NamedAssignment(Token)
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputExpression {
    pub kind: InputExpressionKind,
    pub input: OutputExpression
}

impl Default for InputExpression {
    fn default() -> Self {
        InputExpression {
            kind: InputExpressionKind::OrderedAssignment(0),
            input: OutputExpression::default()
        }
    }
}