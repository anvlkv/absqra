


use super::expressions::output_expression::OutputExpression;
use super::expressions::context_expression::ContextExpression;
use super::expressions::input_expression::InputExpression;
use super::expressions::reference_expression::ReferenceExpression;
use super::expressions::annotation_expression::AnnotationExpression;
use super::expressions::content::Content;
use super::expressions::traits::{*};

use super::errors::ParserError;

use ra_lexer::token::{Token, TokenKind};
use ra_lexer::cursor::Position;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockKind<'a> {
    Program,
    Output(OutputExpression<'a>),
    Input(bool, Option<InputExpression<'a>>),
    Declaration(Option<Token<'a>>),
    Invocation(Option<Token<'a>>, Option<InputExpression<'a>>),
    Reference(Option<ReferenceExpression<'a>>),
    ContextModification(Option<ContextExpression>),
    Content(Content<'a>),
    Annotation(Option<AnnotationExpression<'a>>),
    Union(usize)
}

impl<'a> Default for BlockKind<'a> {
    fn default() -> Self {
        Self::Program
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Block<'a> {
    pub kind: BlockKind<'a>,
    pub children: Vec<Block<'a>>,
    first_token: Token<'a>,
}

impl <'a> Block<'a> {
    pub fn new(first_token: Token<'a>) -> Result<Self, ParserError<'a>> {
        let kind = Self::parse_block_kind(first_token.clone())?;

        Ok(Self {
            kind,
            first_token,
            children: Vec::new()
        })
    }

    fn parse_block_kind(token: Token<'a>) -> Result<BlockKind, ParserError<'a>> {
        if token.kind.is_none() {
            return  Ok(BlockKind::Program)
        }
        
        let token_kind = token.kind.unwrap();

        match token_kind {
            TokenKind::Identifier(_)
            | TokenKind::Float(_)
            | TokenKind::Int(_)
            | TokenKind::OpenParentheses => Ok(BlockKind::Output(OutputExpression::new(token)?)),
            TokenKind::Exclamation => Ok(BlockKind::Invocation(None, None)),
            TokenKind::Plus | TokenKind::Greater => Ok(BlockKind::Input(token_kind == TokenKind::Plus, None)),
            TokenKind::ContentBlock => Ok(BlockKind::Content(Content::new(token)?)),
            TokenKind::At => Ok(BlockKind::Reference(None)),
            TokenKind::Colon => Ok(BlockKind::Declaration(None)),
            TokenKind::OpenCurlyBrace => Ok(BlockKind::ContextModification(None)),
            TokenKind::HashPound => Ok(BlockKind::Annotation(None)),
            _ => Err(ParserError::ExpectedAGotB(token, vec![
                TokenKind::Identifier(""), 
                TokenKind::Float(0.0), 
                TokenKind::Int(0), 
                TokenKind::OpenParentheses, 
                TokenKind::Exclamation, 
                TokenKind::Plus, 
                TokenKind::Greater, 
                TokenKind::ContentBlock, 
                TokenKind::At, 
                TokenKind::Colon, 
                TokenKind::OpenCurlyBrace, 
                TokenKind::HashPound
            ]))
        }
    }
}

impl<'a> Leveled for Block<'a> {
    fn get_level(&self) -> u16{
        match self.kind {
            BlockKind::Program => 0,
            _=> self.first_token.level
        }
    }
}

impl<'a> Positioned for Block<'a> {
    fn get_position(&self) -> (Position, Position) {
        // let first_child = self.children.first();
        let last_child = self.children.last();
        let start_position = self.first_token.position.0;
        let end_position = {
            if last_child.is_some() {
                last_child.unwrap().get_position().1
            }
            else {
                let expression_position = match &self.kind {
                    BlockKind::Output(expression) => Some(expression.get_position().1),
                    BlockKind::Input(_, expression) => {
                        if expression.is_some() {
                            Some(expression.as_ref().unwrap().get_position().1)
                        }
                        else {
                            None
                        }
                    },
                    BlockKind::Invocation(token, expression) => {
                        if expression.is_some() {
                            Some(expression.as_ref().unwrap().get_position().1)
                        }
                        else if token.is_some() {
                            Some(token.unwrap().position.1)
                        }
                        else {
                            None
                        }
                    },
                    BlockKind::Declaration(declared_token) => {
                        if declared_token.is_some() {
                            Some(declared_token.unwrap().position.1)
                        }
                        else {
                            None
                        }
                    },
                    BlockKind::Annotation(expression) => {
                        if expression.is_some() {
                            Some(expression.as_ref().unwrap().get_position().1)
                        }
                        else {
                            None
                        }
                    },
                    BlockKind::Reference(expression) => {
                        if expression.is_some() {
                            Some(expression.as_ref().unwrap().get_position().1)
                        }
                        else {
                            None
                        }
                    },
                    _ => None
                };

                if expression_position.is_some() {
                    expression_position.unwrap()
                }
                else {
                    self.first_token.position.1
                }
            }
        };

        (start_position, end_position)
    }
}

impl<'a> Expandable<'a, Block<'a>, Token<'a>> for Block<'a> {
    fn append_item(self, token: Token<'a>) -> Result<Block<'a>, ParserError<'a>> {
        let mut block = self.clone();
        match self.kind {
            BlockKind::Program 
            | BlockKind::Union(_) 
            | BlockKind::Content(_) => Err(ParserError::InvalidBlock),
            BlockKind::Output(expression) => {
                let updated_expression = expression.append_item(token)?;
                block.kind = BlockKind::Output(updated_expression);
                Ok(block)
            },
            BlockKind::Input(multiple, expression) => {
                let new_expression;
                if expression.is_some() {
                    new_expression = expression.unwrap().append_item(token, None)?;
                }
                else {
                    new_expression = InputExpression::new(token)?;
                }

                block.kind = BlockKind::Input(multiple, Some(new_expression));
                Ok(block)
            },
            BlockKind::Annotation(expression) => {
                let new_expression;
                if expression.is_some() {
                    // let AnnotationExpression(first_token, next) = expression.clone().unwrap();

                    // TODO: should append to existing expression or push to block children
                    // TODO: what are possible annotated blocks

                    new_expression = expression.unwrap().append_item(token)?;
                }
                else {
                    new_expression = AnnotationExpression::new(token)?;
                }

                block.kind = BlockKind::Annotation(Some(new_expression));
                Ok(block)
            },
            BlockKind::Declaration(declared_token) => {
                if declared_token.is_some() {
                    Err(ParserError::InvalidBlock)
                }
                else {
                    block.kind = BlockKind::Declaration(Some(token));
                    Ok(block)
                }
            },
            BlockKind::Invocation(invoked_token, expression) => {
                if invoked_token.is_none() {
                    match token.kind.unwrap() {
                        TokenKind::Identifier(_) => {
                            block.kind = BlockKind::Invocation(Some(token), None);
                        },
                        _ => return Err(ParserError::ExpectedAGotB(token, vec![TokenKind::Identifier("")]))
                    }
                }
                else if expression.is_none() {
                    block.kind = BlockKind::Invocation(invoked_token, Some(InputExpression::new(token)?));
                }
                else {
                    block.kind = BlockKind::Invocation(invoked_token, Some(expression.unwrap().append_item(token, None)?))
                }

                Ok(block)
            },
            BlockKind::Reference(expression) => {
                let new_expression;
                if expression.is_some() {
                    new_expression = expression.unwrap().append_item(token)?;
                }
                else {
                    new_expression = ReferenceExpression::new(token)?;
                }

                block.kind = BlockKind::Reference(Some(new_expression));
                Ok(block)
            },
            BlockKind::ContextModification(expression) => {
                let new_expression;
                if expression.is_some() {
                    new_expression = expression.unwrap().append_item(token)?;
                }
                else {
                    new_expression = ContextExpression::new(token)?;
                }

                block.kind = BlockKind::ContextModification(Some(new_expression));
                Ok(block)
            }
        }
    }
}