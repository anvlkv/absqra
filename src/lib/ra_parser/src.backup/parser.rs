use crate::expressions::context_expression::{ContextExpression, ContextExpressionMemberKind, ContextExpressionMember};
use ra_lexer::token::{RaToken, TokenKind};
use ra_lexer::tokenize;
use ra_lexer::errors::LexerError;

use super::block::{Block, BlockKind};
use super::cursor::Cursor;
use super::errors::ParserError;
use super::expressions::traits::{Leveled, Expandable, Positioned};
use failure::Backtrace;

pub fn parse<'a>(input: &'a str) -> Result<Block<'a>, (Vec<ParserError>, Block<'a>)> {
    let stream = tokenize(input);
    
    let mut errors: Vec<ParserError> = Vec::new();

    let program = {
        let mut block = Block::new(RaToken::default()).unwrap();

        let mut tokens_cursor = Cursor::new(
            stream
            .take_while(|r| r.is_ok())
            .map(|r| r.unwrap())
            .filter(|tok| tok.kind.unwrap() != TokenKind::Comment)
        );
        loop {
            if !tokens_cursor.is_eof() {
                match tokens_cursor.advance_block() {
                    Ok(blk) => block.children.push(blk),
                    Err(es) => errors.extend(es.into_iter()),
                }
            } else {
                break;
            }
        }

        block
    };

    if errors.len() > 0 {
        Err((errors, program))
    } else {
        Ok(program)
    }
}

impl<'token, I> Cursor<'token, I>
where
    I: Iterator<Item = RaToken<'token>>,
{
    fn advance_block<'a>(&'a mut self) -> Result<Block<'token>, Vec<ParserError>> {
        let first_token = self.bump();
        let mut errors: Vec<ParserError> = Vec::new();
        match first_token {
            Some(token) => {
                self.parse_block(token)
            },
            None => {
                errors.push(ParserError::UnexpectedEndOfInput(self.position, Backtrace::new()));
                Err(errors)
            }
        }
    }

    fn parse_block(
        &mut self,
        first_token: RaToken<'token>,
    ) -> Result<Block<'token>, Vec<ParserError>> {
        let mut errors: Vec<ParserError> = Vec::new();
        let block = Block::new(first_token)?;

        if let (Some(blk), mut errs) =
            self.check_parse_block_expression(block, &mut errors)
        {
            if let (Some(blk), mut errs) = self.check_parse_block_children(blk, &mut errs) {
                if let (Some(blk), errs) = self.check_parse_union(blk, &mut errs) {
                    if errs.len() == 0 {
                        return Ok(blk);
                    }
                }
            }
        }

        Err(errors)
    }

    fn check_parse_block_expression<'errors>(
        &mut self,
        mut block: Block<'token>,
        errors: &'errors mut Vec<ParserError>,
    ) -> (Option<Block<'token>>, &'errors mut Vec<ParserError>) {
        let mut lvl = block.get_level();
        let mut line_number = (block.get_position().0).0;
        while self.first_ahead().is_some()
            && self.first_ahead().unwrap().level == lvl
            && (self.first_ahead().unwrap().position.0).0 == line_number {
                match block.clone().append_item(self.bump().unwrap()) {
                    Ok(blk) => {
                        block = blk;

                        // // TODO: ugly, move this logic elsewhere

                        if let BlockKind::ContextModification(Some(ContextExpression(target, source))) = block.clone().kind { 
                            if ContextExpressionMember::Target(ContextExpressionMemberKind::MSpecifier) == target && source.is_none() {
                                if let (Some(mut blk), mut errs) = self.check_parse_block_children(block.clone(), errors) {
                                    let close_curly_brace = self.bump().unwrap();
                                    assert_eq!(close_curly_brace.kind.unwrap(), TokenKind::CloseCurlyBrace);
                                    block = blk;
                                    lvl = close_curly_brace.level;
                                    line_number = (close_curly_brace.position.0).0;
                            //         // let updated_expression = ContextExpression(target, None).append_item(self.bump().unwrap())?;

                            //         // blk.kind = BlockKind::ContextModification(Some(updated_expression));
                            //         // self.check_parse_block_expression(blk, errs)
                                }
                            }
                            else if source.is_some() && ContextExpressionMember::Source(ContextExpressionMemberKind::MSpecifier) == source.unwrap() {
                                if let (Some(blk), mut errs) = self.check_parse_block_children(block.clone(), errors) {
                                    assert_eq!(self.bump().unwrap().kind.unwrap(), TokenKind::CloseCurlyBrace);
                                    assert_eq!(self.bump().unwrap().kind.unwrap(), TokenKind::CloseCurlyBrace);
                                    block = blk;
                                    // return Ok(blk);
                                }
                            }
                            println!("{:?}", block);
                        }

                        // // end TODO
                    },
                    Err(e) => {
                        errors.push(e)
                    }
                }
        }

        (Some(block), errors)
    }

    fn check_parse_block_children<'errors>(
        &mut self,
        mut block: Block<'token>,
        errors: &'errors mut Vec<ParserError>,
    ) -> (Option<Block<'token>>, &'errors mut Vec<ParserError>) {
        
        while !self.is_eof() && self.first_ahead().unwrap().level == block.get_level() + 1 {
            match self.advance_block() {
                Ok(child) => {
                    block.children.push(child);
                }
                Err(errs) => {
                    errors.extend(errs);
                }
            }
        }

        if errors.len() == 0 {
            (Some(block), errors)
        } else {
            (None, errors)
        }
    }

    fn check_parse_union<'errors>(
        &mut self,
        block: Block<'token>,
        errors: &'errors mut Vec<ParserError>,
    ) -> (Option<Block<'token>>, &'errors mut Vec<ParserError>) {
        let mut union_block = block.clone();
        let lvl = block.get_level();
        let mut union_size = 0;
        while !self.is_eof()
            && self.first_ahead().unwrap().kind.unwrap() == TokenKind::Coma
            && self.first_ahead().unwrap().level == lvl
        {
            assert!(self.bump().unwrap().kind.unwrap() == TokenKind::Coma);
            match self.advance_block() {
                Ok(blk) => {
                    union_size += 1;
                    union_block.children.push(blk);
                }
                Err(errs) => {
                    errors.extend(errs);
                }
            }
        }
        
        if union_size > 0 {
            union_block.kind = BlockKind::Union(union_size);
            (Some(union_block), errors)
        } else {
            (Some(block), errors)
        }
    }
}
