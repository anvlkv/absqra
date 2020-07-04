use ra_lexer::token::{Token, TokenKind};
use ra_lexer::tokenize;

use super::block::{Block, BlockKind};
use super::cursor::Cursor;
use super::errors::ParserError;
use super::expressions::traits::{Leveled, Expandable, Positioned};
use failure::Backtrace;

pub fn parse<'a>(input: &'a str) -> Result<Block<'a>, (Vec<ParserError>, Block<'a>)> {
    let stream = tokenize(input).filter(|tok| tok.kind.unwrap() != TokenKind::Comment);
    let mut errors: Vec<ParserError> = Vec::new();
    let program = {
        let mut block = Block::new(Token::default()).unwrap();
        let mut tokens_cursor = Cursor::new(stream);
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
    I: Iterator<Item = Token<'token>>,
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
        first_token: Token<'token>,
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
        let lvl = block.get_level();
        let line_number = (block.get_position().0).0;
        while self.first_ahead().is_some()
            && self.first_ahead().unwrap().level == lvl
            && (self.first_ahead().unwrap().position.0).0 == line_number {
                match block.clone().append_item(self.bump().unwrap()) {
                    Ok(blk) => {
                        block = blk;
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
