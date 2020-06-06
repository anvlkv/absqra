use ra_lexer::token::{Token, TokenKind};
use ra_lexer::tokenize;

use super::block::{Block, BlockKind};
use super::cursor::Cursor;
use super::errors::ParserError;
use super::expression::Expression;

pub fn parse<'a>(input: &'a str) -> Result<Block<'a>, Vec<ParserError>> {
    let stream = tokenize(input);
    let mut errors: Vec<ParserError> = Vec::new();
    let program = {
        let mut children = Vec::new();
        let mut tokens_cursor = Cursor::new(stream);
        loop {
            if !tokens_cursor.is_eof() {
                match tokens_cursor.advance_block() {
                    Ok(blk) => children.push(blk),
                    Err(es) => errors.extend(es.into_iter()),
                }
            } else {
                break;
            }
        }

        Block {
            children,
            level: 0,
            kind: Some(BlockKind::Program),
            ..Default::default()
        }
    };

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(program)
    }
}

impl<'token, I> Cursor<'token, I>
where
    I: Iterator<Item = Token<'token>>,
{
    fn advance_block<'a>(&'a mut self) -> Result<Block<'token>, Vec<ParserError<'token>>> {
        let first_token = self.bump();
        let mut errors: Vec<ParserError<'token>> = Vec::new();
        match first_token {
            Some(token) => match token.kind.unwrap() {
                TokenKind::Identifier(_)
                | TokenKind::Float(_)
                | TokenKind::Int(_)
                | TokenKind::OpenParentheses => self.parse_block(token, BlockKind::Output),
                TokenKind::Colon => self.parse_block(token, BlockKind::Declaration(None)),
                TokenKind::Exclamation => self.parse_block(token, BlockKind::RuleInvocation),
                TokenKind::Plus | TokenKind::Greater => self.parse_block(token, BlockKind::Input),
                TokenKind::OpenCurlyBrace => self.parse_block(token, BlockKind::ContextDeclaration),
                TokenKind::ContentBlock => self.parse_block(token, BlockKind::Content),
                TokenKind::At => self.parse_block(token, BlockKind::Reference),
                TokenKind::Comment => self.advance_block(),
                _ => {
                    errors.push(ParserError::UnexpectedToken(token));
                    Err(errors)
                }
            },
            None => {
                errors.push(ParserError::UnexpectedEndOfInput(self.position));
                Err(errors)
            }
        }
    }

    fn parse_block(
        &mut self,
        first_token: Token<'token>,
        kind: BlockKind,
    ) -> Result<Block<'token>, Vec<ParserError<'token>>> {
        let mut errors: Vec<ParserError<'token>> = Vec::new();
        let block = Some(Block::new(first_token, kind));

        if let (Some(blk), mut errs) =
            self.check_parse_block_expression(block.unwrap(), &mut errors)
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
        errors: &'errors mut Vec<ParserError<'token>>,
    ) -> (Option<Block<'token>>, &'errors mut Vec<ParserError<'token>>) {
        if let Some(token) = self.first_ahead() {
            let first_block_token = block.expression.0;
            if token.level == first_block_token.level
                && (token.position.0).0 == (first_block_token.position.0).0
            {
                block.expression = Expression::new(self.bump().unwrap());
                while !self.is_eof()
                    && self.first_ahead().is_some()
                    && (self.first_ahead().unwrap().position.0).0
                        == (first_block_token.position.0).0
                {
                    match block.clone().expression.append_token(self.bump().unwrap()) {
                        Ok(expression) => {
                            block.expression = expression;
                        }
                        Err(e) => {
                            errors.push(e);
                            break;
                        }
                    };
                }
            }
        }
        (Some(block), errors)
    }

    fn check_parse_block_children<'errors>(
        &mut self,
        mut block: Block<'token>,
        errors: &'errors mut Vec<ParserError<'token>>,
    ) -> (Option<Block<'token>>, &'errors mut Vec<ParserError<'token>>) {
        let first_block_token = block.expression.0;
        while !self.is_eof() && self.first_ahead().unwrap().level == first_block_token.level + 1 {
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
        errors: &'errors mut Vec<ParserError<'token>>,
    ) -> (Option<Block<'token>>, &'errors mut Vec<ParserError<'token>>) {
        let first_block_token = block.expression.0;
        let mut union_block = Block::default();
        let mut union_size = 0;
        while !self.is_eof()
            && self.first_ahead().unwrap().kind.unwrap() == TokenKind::Coma
            && self.first_ahead().unwrap().level == first_block_token.level
        {
            assert!(self.bump().unwrap().kind.unwrap() == TokenKind::Coma);
            match self.advance_block() {
                Ok(blk) => {
                    union_size += 1;
                    union_block.children.push(blk);
                    union_block.kind = Some(BlockKind::Union(union_size))
                }
                Err(errs) => {
                    errors.extend(errs);
                }
            }
        }

        if union_size > 0 {
            (Some(union_block), errors)
        } else {
            (Some(block), errors)
        }
    }
}
