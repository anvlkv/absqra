// use std::rc::Rc;
// use std::borrow::Borrow;
use ra_lexer::token::{Token, TokenKind};
use ra_lexer::cursor::Position;
// use super::block::Block;
use super::errors::ParserError;
use super::traits::{*};

#[derive(Debug, Clone, PartialEq, Copy)]
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ComparisonOperation {
    EqCompare,
    GtCompare,
    LsCompare,
    GtEqCompare,
    LsEqCompare,
    NEqCompare,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LogicOperation {
    AND, // &
    OR, // |
    NOT, // !
    NAND, // !&
    NOR, // !!
    XOR, // ||
    XNOR // !|
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum OperationKind {
    LogicOperation(LogicOperation),
    MathOperation(MathOperation),
    ComparisonOperation(ComparisonOperation),
    Assign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionMember<'a> {
    Identifier(Token<'a>),
    Literal(Token<'a>),
    OutputExpression(bool, Option<OutputExpression<'a>>),
    Nil
}

impl<'a> Default for ExpressionMember<'a> {
    fn default() -> Self {
        Self::Nil
    }
}


impl <'a> ExpressionMember<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError<'a>> {
        match token.kind.unwrap() {
            TokenKind::Identifier(_) => Ok(ExpressionMember::Identifier(token)),
            TokenKind::Int(_) |
            TokenKind::Float(_) |
            TokenKind::StringLiteral(_) => Ok(ExpressionMember::Literal(token)),
            TokenKind::OpenParentheses => Ok(ExpressionMember::OutputExpression(false, None)),
            _ => Err(ParserError::ExpectedAGotB(token, vec![TokenKind::Identifier(""), TokenKind::Int(0), TokenKind::Float(0.0), TokenKind::StringLiteral(""), TokenKind::OpenParentheses]))
        }
    }
}

impl <'a> Leveled for ExpressionMember<'a> {
    fn get_level(&self) -> u16 {
        match self {
            ExpressionMember::Nil => 0,
            ExpressionMember::Literal(token) |
            ExpressionMember::Identifier(token) => token.level,
            ExpressionMember::OutputExpression(_, expression) => expression.as_ref().unwrap().get_level()
        }
    }
}

impl <'a> Expandable<'a, ExpressionMember<'a>, Token<'a>> for ExpressionMember<'a> {
    fn append_item(self, token: Token<'a>) -> Result<ExpressionMember<'a>, ParserError<'a>> {
        match self {
            ExpressionMember::OutputExpression(open, expression) => {
                if open {
                    match expression {
                        Some(e) => {
                            let updated_nested_expression = e.append_item(token)?;
                            Ok(ExpressionMember::OutputExpression(false, Some(updated_nested_expression)))
                        },
                        None => {
                            Ok(ExpressionMember::OutputExpression(false, Some(OutputExpression::new(token)?)))
                        }
                    }
                }
                else {
                    Err(ParserError::InvalidExpression(token.position.0))
                }
            },
            _ => Err(ParserError::InvalidExpression(token.position.0))
        }
    }
}

impl <'a> Positioned  for ExpressionMember<'a> {
    fn get_position(&self) -> (Position, Position) {
        match self {
            ExpressionMember::Nil => (Position::default(), Position::default()),
            ExpressionMember::Literal(token) |
            ExpressionMember::Identifier(token) => token.position,
            ExpressionMember::OutputExpression(_, expression) => expression.as_ref().unwrap().get_position()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OutputExpression<'a>(
    pub Box<ExpressionMember<'a>>, 
    pub Option<OperationKind>, 
    pub Option<Box<ExpressionMember<'a>>>
);

impl <'a> Leveled for OutputExpression<'a> {
    fn get_level(&self) -> u16 {
        let OutputExpression(first_member, _, _) = self;
        first_member.get_level()
    }
}

impl <'a> Expandable<'a, OutputExpression<'a>, Token<'a>> for OutputExpression<'a> {
    fn append_item(self, token: Token<'a>) -> Result<OutputExpression<'a>, ParserError<'a>> {
        let OutputExpression(first_member, op, last_member) = self;
        // TODO: can this be done without matching?

        match first_member.as_ref() {
            ExpressionMember::OutputExpression(open, expression) => {
                if *open {
                    if token.kind.unwrap() == TokenKind::CloseParentheses {
                        if expression.is_some() {
                            return Ok(OutputExpression(Box::new(ExpressionMember::OutputExpression(true, expression.clone())), None, None))
                        }
                        else {
                            return Err(ParserError::InvalidExpression(token.position.0))
                        }
                    }
                    else {
                        
                    }
                }
            },
            _ => {}
        }

        // first_member.

        if op.is_none() {
            match Self::parse_operation_first_token(token) {
                Some(operation) => Ok(OutputExpression(first_member.clone(), Some(operation), None)),
                None => Err(ParserError::UnexpectedToken(token))
            }
        }
        else {
            match Self::parse_operation_second_token(op.unwrap(), token) {
                Some(operation) => Ok(OutputExpression(first_member.clone(), Some(operation), None)),
                None => {
                    if last_member.is_none() {
                        Ok(OutputExpression(first_member.clone(), op.clone(), Some(Box::new(ExpressionMember::new(token)?))))
                    }
                    else {
                        let child_expression = last_member.unwrap();
                        let child_member = child_expression.append_item(token)?;
                        Ok(OutputExpression(first_member.clone(), op.clone(), Some(Box::new(child_member))))
                    }
                }
            }
        }
    }
}

impl <'a> Positioned for OutputExpression<'a> {
    fn get_position(&self) -> (Position, Position) {
        let OutputExpression(first_member, _, last_member) = self;
        
        let start_position = first_member.get_position().0;
        let end_position = {
            if last_member.is_some() {
                last_member.clone().unwrap().get_position().1
            }
            else {
                first_member.get_position().1
            }
        };

        (start_position, end_position)
    }
}

impl <'a> OutputExpression<'a> {
    pub fn new(token: Token<'a>) -> Result<Self, ParserError<'a>> {
        let left_member = ExpressionMember::new(token)?;

        Ok(Self(Box::new(left_member), None, None))
    }


    fn parse_operation_first_token(token: Token) -> Option<OperationKind> {
        match token.kind.unwrap() {
            TokenKind::Plus => Some(OperationKind::MathOperation(MathOperation::Sum)),
            TokenKind::Minus => Some(OperationKind:: MathOperation(MathOperation::Subtract)),
            TokenKind::Asterisk => Some(OperationKind:: MathOperation(MathOperation::Multiply)),
            TokenKind::Slash => Some(OperationKind:: MathOperation(MathOperation::Divide)),
            TokenKind::Percent => Some(OperationKind:: MathOperation(MathOperation::Reminder)),
            TokenKind::Power => Some(OperationKind:: MathOperation(MathOperation::Power)),
            
            TokenKind::Greater => Some(OperationKind::ComparisonOperation(ComparisonOperation::GtCompare)),
            TokenKind::Less => Some(OperationKind::ComparisonOperation(ComparisonOperation::LsCompare)),
            
            TokenKind::Ampersand => Some(OperationKind::LogicOperation(LogicOperation::AND)),
            TokenKind::Pipe => Some(OperationKind::LogicOperation(LogicOperation::OR)),
            TokenKind::Exclamation=> Some(OperationKind::LogicOperation(LogicOperation::NOT)),
            
            TokenKind::Equals => Some(OperationKind::Assign),
            
            _ => None
        }
    }

    fn parse_operation_second_token(op: OperationKind, token: Token) -> Option<OperationKind> {
        match token.kind.unwrap() {
            TokenKind::Equals => {
                match op {
                    OperationKind::MathOperation(m_op) => {
                        match m_op {
                            MathOperation::Sum => Some(OperationKind::MathOperation(MathOperation::AddAssign)),
                            MathOperation::Subtract => Some(OperationKind::MathOperation(MathOperation::SubtractAssign)),
                            _ => None
                        }
                    },
                    OperationKind::ComparisonOperation(c_op) => {
                        match c_op {
                            ComparisonOperation::GtCompare => Some(OperationKind::ComparisonOperation(ComparisonOperation::GtEqCompare)),
                            ComparisonOperation::LsCompare => Some(OperationKind::ComparisonOperation(ComparisonOperation::LsEqCompare)),
                            _ => None
                        }
                    },
                    OperationKind::LogicOperation(l_op) => {
                        match l_op {
                            LogicOperation::NOT => Some(OperationKind::ComparisonOperation(ComparisonOperation::NEqCompare)),
                            _ => None
                        }
                    },
                    OperationKind::Assign => Some(OperationKind::ComparisonOperation(ComparisonOperation::EqCompare)),
                }
            },
            TokenKind::Ampersand => {
                match op {
                    OperationKind::LogicOperation(l_op) => {
                        match l_op {
                            LogicOperation::NOT => Some(OperationKind::LogicOperation(LogicOperation::NAND)),
                            _ => None
                        }
                    },
                    _ => None
                }
            },
            TokenKind::Pipe => {
                match op {
                    OperationKind::LogicOperation(l_op) => {
                        match l_op {
                            LogicOperation::OR => Some(OperationKind::LogicOperation(LogicOperation::XOR)),
                            LogicOperation::NOT => Some(OperationKind::LogicOperation(LogicOperation::XNOR)),
                            _ => None
                        }
                    },
                    _ => None
                }
            },
            TokenKind::Exclamation => {
                match op {
                    OperationKind::LogicOperation(l_op) => {
                        match l_op {
                            LogicOperation::NOT => Some(OperationKind::LogicOperation(LogicOperation::NOR)),
                            _ => None
                        }
                    },
                    _ => None
                }
            },
            _ => None
        }
    }
}


// #[derive(Debug, Clone, PartialEq)]
// pub enum ExpressionMember<'a> {
//     Token(Token<'a>),
//     OutputExpression(OutputExpression<'a>, bool),
// }

// #[derive(Debug, Clone, PartialEq, Default)]
// pub struct OutputExpression<'a> {
//     pub left: Option<Rc<ExpressionMember<'a>>>,
//     pub operation: Option<OperationKind>,
//     pub right: Option<Rc<ExpressionMember<'a>>>,
//     pub tokens: Vec<Token<'a>>
// }

// impl <'a> OutputExpression<'a> {
//     pub fn from_token(token: Token<'a>) -> Self {
//         let left = {
//             match OutputExpression::parse_left(token) {
//                 Some(t) => Some(t),
//                 None => {
//                     match OutputExpression::parse_bracket(token) {
//                         Some((closing, expression)) => {
//                             if !closing {
//                                 Some(expression)
//                             }
//                             else {
//                                 None
//                             }
//                         },
//                         None => None
//                     }
//                 }
//             }
//         };
        
//         Self {
//             left,
//             tokens: vec![token],
//             ..OutputExpression::default()
//         }
//     }

//     pub fn is_complete(&self) -> bool {
//         self.left.is_some() 
//         && OutputExpression::check_is_expression_member_complete(self.left.as_ref().unwrap())
//         && self.operation.is_some()
//         && self.right.is_some()
//         && OutputExpression::check_is_expression_member_complete(self.right.as_ref().unwrap())
//     }


//     fn check_is_expression_member_complete(em: &ExpressionMember) -> bool {
//         match em {
//             ExpressionMember::Token(_) => true,
//             ExpressionMember::OutputExpression(child, complete) => child.is_complete() && *complete,
//             ExpressionMember::ContextExpression(blk, complete) => blk.is_complete() && *complete
//         }
//     }

//     pub fn append_token(&mut self, token: Token<'a>) -> Option<Vec<ParserError>> {
//         self.tokens.push(token);

//         if self.left.is_some() {
//             if OutputExpression::check_is_expression_member_complete(self.left.unwrap().as_ref()) {
//                 if self.operation.is_some() && self.right.is_none() {
//                     match OutputExpression::parse_operation_second_token(self.operation.unwrap(), token) {
//                         Some(op) => {
//                             self.operation = Some(op);
//                             None
//                         },
//                         None => {
//                             match OutputExpression::parse_right(token) {
//                                 Some(right) => {
//                                     self.right = Some(right);
//                                     None
//                                 }
//                                 None => {
//                                     match OutputExpression::parse_bracket(token) {
//                                         Some((closing, expression)) => {
//                                             if !closing {
//                                                 self.right = Some(expression);
//                                                 None
//                                             }
//                                             else {
//                                                 Some(vec![ParserError::UnexpectedToken])
//                                             }
//                                         },
//                                         None => {
//                                             Some(vec![ParserError::InvalidExpression])
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 else if self.right.is_some() && self.operation.is_some(){
//                     if OutputExpression::check_is_expression_member_complete(self.right.unwrap().as_ref()) {
//                         match OutputExpression::parse_operation_first_token(token) {
//                             Some(op) => {
//                                 self.right = Some(Rc::new(ExpressionMember::OutputExpression(OutputExpression {
//                                     left: self.right.clone(),
//                                     operation: Some(op),
//                                     ..Default::default()
//                                 }, false)));
//                                 None
//                             },
//                             None => {
//                                 Some(vec![ParserError::InvalidExpression])
//                             }
//                         }
//                     }
//                     else {
//                         match OutputExpression::parse_bracket(token) {
//                             Some((closing, expression)) => {
//                                 if !closing {
//                                     self.right = Some(expression);
//                                     None
//                                 }
//                                 else {
//                                     match self.right.unwrap().borrow() {
//                                         ExpressionMember::OutputExpression(expr, _) => {
//                                             match expression.as_ref() {
//                                                 ExpressionMember::OutputExpression(_, _) => {
//                                                     self.right = Some(Rc::new(ExpressionMember::OutputExpression(expr.clone(), true)));
//                                                     None
//                                                 }
//                                                 _ => {
//                                                     Some(vec![ParserError::InvalidExpression])        
//                                                 }
//                                             }
//                                         },
//                                         ExpressionMember::ContextExpression(blk, _) => {
//                                             match expression.as_ref() {
//                                                 ExpressionMember::ContextExpression(_, _) => {
//                                                     self.right = Some(Rc::new(ExpressionMember::ContextExpression(blk.clone(), true)));
//                                                     None
//                                                 }
//                                                 _ => {
//                                                     Some(vec![ParserError::InvalidExpression])        
//                                                 }
//                                             }
//                                         },
//                                         _ => {
//                                             Some(vec![ParserError::InvalidExpression])
//                                         }
//                                     }
//                                 }
//                             },
//                             None => {
//                                 match self.right.unwrap().borrow() {
//                                     ExpressionMember::OutputExpression(mut expr, _) => {
//                                         expr.append_token(token)
//                                     },
//                                     ExpressionMember::ContextExpression(mut blk, _) => {
//                                         blk.expression.append_token(token)
//                                     },
//                                     _ => {
//                                         Some(vec![ParserError::InvalidExpression])
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 else {
//                     match OutputExpression::parse_operation_first_token(token) {
//                         Some(op) => {
//                             self.operation = Some(op);
//                             None
//                         }
//                         None => {
//                             Some(vec![ParserError::InvalidExpression])
//                         }
//                     }
//                 }
//             }
//             else {
//                 match self.left.unwrap().borrow() {
//                     ExpressionMember::OutputExpression(mut expr, _) => {
//                         expr.append_token(token)
//                     },
//                     ExpressionMember::ContextExpression(mut blk, _) => {
//                         blk.expression.append_token(token)
//                     },
//                     _ => {
//                         Some(vec![ParserError::InvalidExpression])
//                     },
//                 }
//             }
//         }
//         else {
//             match OutputExpression::parse_left(token) {
//                 Some(em) => {
//                     self.left = Some(em);
//                     None
//                 },
//                 None => {
//                     match OutputExpression::parse_bracket(token) {
//                         Some((closing, nested_expression)) => {
//                             if !closing {
//                                 self.left = Some(nested_expression);
//                                 None
//                             }
//                             else {
//                                 Some(vec![ParserError::UnexpectedToken])
//                             }
//                         }
//                         None => Some(vec![ParserError::InvalidExpression])
//                     }
//                 }
//             }
//         }
//     }

    

//     fn parse_left(token: Token) -> Option<Rc<ExpressionMember>> {
//         match token.kind.unwrap() {
//             TokenKind::Identifier(_)
//             | TokenKind::Int(_)
//             | TokenKind::Float(_)
//             | TokenKind::StringLiteral(_) => Some(Rc::new(ExpressionMember::Token(token))),
//             _ => None
//         }
//     }

//     fn parse_right(token: Token) -> Option<Rc<ExpressionMember>> {
//         match token.kind.unwrap() {
//             TokenKind::Identifier(_)
//             | TokenKind::Int(_)
//             | TokenKind::Float(_)
//             | TokenKind::StringLiteral(_) => Some(Rc::new(ExpressionMember::Token(token))),
//             _ => None
//         }
//     }

//     fn parse_bracket(token: Token) -> Option<(bool, Rc<ExpressionMember>)> {
//         match token.kind.unwrap() {
//             TokenKind::OpenParentheses => Some((false, Rc::new(ExpressionMember::OutputExpression(OutputExpression::default(), false)))),
//             TokenKind::OpenCurlyBrace => Some((false, Rc::new(ExpressionMember::ContextExpression(Block::default(), false)))),

//             TokenKind::CloseParentheses => Some((true, Rc::new(ExpressionMember::OutputExpression(OutputExpression::default(), true)))),
//             TokenKind::CloseCurlyBrace => Some((true, Rc::new(ExpressionMember::ContextExpression(Block::default(), true)))),

//             _ => None
//         }
//     }

// }

// use ra_lexer::token::{Token, TokenKind};
// // use std::rc::Rc;
// use super::errors::ParserError;
// // use std::slice::SliceIndex;



// #[derive(Debug, Clone, PartialEq, Copy)]
// pub enum ExpressionMember{
//     OutputExpression(OutputExpression, bool),
//     Identifier(Token),
//     Literal(Token),
//     Operation(OperationKind),
//     None
// }

// #[derive(Debug, Clone, PartialEq, Copy)]
// pub struct OutputExpression {
//     members: [Box<ExpressionMember>; 3]
// }


// impl Default for OutputExpression {
//     fn default() -> Self {
//         OutputExpression {
//             members: [Box::new(ExpressionMember::None), Box::new(ExpressionMember::None), Box::new(ExpressionMember::None)]
//         }
//     }
// }

// impl OutputExpression {
//     pub fn new() -> OutputExpression {
//         OutputExpression::default()
//     }

//     pub fn append_token(&mut self, token: Token) -> Result<(), ParserError> {
//          match token.kind {
//             TokenKind::OpenParentheses => {
//                 self.append_expression_member(ExpressionMember::OutputExpression(OutputExpression::new(), false))
//             },
//             TokenKind::CloseParentheses => {
//                 self.close_last_expression_member()
//             }
//             TokenKind::Identifier => {
//                 self.append_expression_member(ExpressionMember::Identifier(token))
//             }
//             TokenKind::Float(_) |
//             TokenKind::Int(_) |
//             TokenKind::StringLiteral => {
//                 self.append_expression_member(ExpressionMember::Literal(token))
//             },
//             TokenKind::Plus |
//             TokenKind::Minus |
//             TokenKind::AddAssign |
//             TokenKind::SubtractAssign |
//             TokenKind::Asterisk |
//             TokenKind::Slash |
//             TokenKind::Percent |
//             TokenKind::Power |
//             TokenKind::Equals |
//             TokenKind::Greater |
//             TokenKind::GreaterOrEquals |
//             TokenKind::Less |
//             TokenKind::LessOrEquals |
//             TokenKind::NotEquals => {
//                 self.append_operation(token)
//             },
//             TokenKind::Ampersand |
//             TokenKind::Pipe |
//             TokenKind::Exclamation => {
//                 self.append_logical_operation(token)
//             },
//             _ => Err(ParserError::UnexpectedToken(token))
//         }
//     }

//     fn append_expression_member(&mut self, expression_member: ExpressionMember) -> Result<(), ParserError> {
//         for (i, member) in self.members.iter().enumerate() {
//             match member.as_ref() {
//                 &ExpressionMember::None => {
//                     self.members[i] = Box::new(expression_member);
//                     return Ok(())
//                 },
//                 &ExpressionMember::OutputExpression(mut expression, is_complete) => {
//                     if !is_complete {
//                         expression.append_expression_member(expression_member);
//                         return Ok(())
//                     }
//                 }
//                 _ => {}
//             }
//         }

//         Err(ParserError::InvalidExpression)
//     }

//     fn close_last_expression_member(&mut self) -> Result<(), ParserError> {
//         for (i, member) in self.members.iter().enumerate() {
//             match member.as_mut() {
//                 &mut ExpressionMember::OutputExpression(mut expression, is_complete) => {
//                     if !is_complete {
//                         if !expression.is_nested_expression_complete() {
//                             return expression.close_last_expression_member()
//                         }
//                         else {
//                             self.members[i] = Box::new(ExpressionMember::OutputExpression(expression, true));
//                             return Ok(())
//                         }
//                     }
//                 }
//                 _ => {}
//             }
//         }

//         Err(ParserError::InvalidExpression)
//     }

//     fn is_nested_expression_complete(&self) -> bool {
//         for (i, member) in self.members.iter().enumerate() {
//             match member.as_ref() {
//                 &ExpressionMember::OutputExpression(expression, is_complete) => {
//                     return expression.is_nested_expression_complete() && is_complete
//                 },
//                 _ => {}
//             }
//         }
//         true
//     }

//     fn append_operation(&mut self, token: Token) -> Result<(), ParserError> {
//         for (i, member) in self.members.iter().enumerate() {
//             match member.as_mut() {
//                 &mut ExpressionMember::None => {
//                     self.members[i] = Box::new(ExpressionMember::Operation(match &token.kind {
//                         TokenKind::Plus => OperationKind::Sum,
//                         TokenKind::Minus => OperationKind::Subtract,
//                         TokenKind::AddAssign => OperationKind::AddAssign,
//                         TokenKind::SubtractAssign => OperationKind::SubtractAssign,
//                         TokenKind::Asterisk => OperationKind::Multiply,
//                         TokenKind::Slash => OperationKind::Divide,
//                         TokenKind::Percent => OperationKind::Reminder,
//                         TokenKind::Power => OperationKind::Power,
//                         TokenKind::Equals => OperationKind::EqCompare,
//                         TokenKind::Greater => OperationKind::GtCompare,
//                         TokenKind::GreaterOrEquals => OperationKind::GtEqCompare,
//                         TokenKind::Less => OperationKind::LsCompare,
//                         TokenKind::LessOrEquals => OperationKind::LsEqCompare,
//                         TokenKind::NotEquals => OperationKind::NEqCompare,
//                         _ => return Err(ParserError::UnexpectedToken(token))
//                     }));

//                     return Ok(())
//                 },
//                 &mut ExpressionMember::OutputExpression(mut expression, is_complete) => {
//                     if !is_complete {
//                         return expression.append_operation(token)
//                     }
//                 }
//                 _ => {}
//             }
//         }

//         Err(ParserError::InvalidExpression)
//     }

//     fn append_logical_operation(&mut self, token: Token) -> Result<(), ParserError>{
//         for (i, mut member) in self.members.iter().enumerate() {
//             match member.as_ref() {
//                 &ExpressionMember::None => {
//                     self.members[i] = Box::new(ExpressionMember::Operation(match &token.kind {
//                         TokenKind::Ampersand => OperationKind::AND,
//                         TokenKind::Pipe => OperationKind::OR,
//                         TokenKind::Exclamation => OperationKind::NOT,
//                         _=> return Err(ParserError::UnexpectedToken(token))
//                     }));
//                     return Ok(())
//                 },
//                 &ExpressionMember::OutputExpression(mut expression, is_complete) => {
//                     if !is_complete {
//                         return expression.append_logical_operation(token)
//                     }
//                 },
//                 &ExpressionMember::Operation(operation) => {
//                     match operation {
//                         OperationKind::NOT => {
//                             match &token.kind {
//                                 TokenKind::Ampersand => {
//                                     self.members[i] = Box::new(ExpressionMember::Operation(OperationKind::NAND));
//                                     return Ok(())
//                                 },
//                                 TokenKind::Exclamation => {
//                                     self.members[i] = Box::new(ExpressionMember::Operation(OperationKind::NOR));
//                                     return Ok(())
//                                 },
//                                 TokenKind::Pipe => {
//                                     self.members[i] = Box::new(ExpressionMember::Operation(OperationKind::XNOR));
//                                     return Ok(())
//                                 },
//                                 _ => {}
//                             }
//                         },
//                         OperationKind::OR => {
//                             match &token.kind {
//                                 &TokenKind::Pipe => {
//                                     self.members[i] = Box::new(ExpressionMember::Operation(OperationKind::XOR));
//                                     return Ok(())
//                                 },
//                                 _ => {}
//                             }
//                         },
//                         _ => {}
//                     }
//                 }
//                 _=> {}
//             }
//         }
//         Err(ParserError::InvalidExpression)
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum InputExpressionKind {
//     OrderedAssignment(usize),
//     NamedAssignment(Token)
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct InputExpression {
//     pub kind: InputExpressionKind,
//     pub input: OutputExpression
// }

// impl Default for InputExpression {
//     fn default() -> Self {
//         InputExpression {
//             kind: InputExpressionKind::OrderedAssignment(0),
//             input: OutputExpression::default()
//         }
//     }
// }