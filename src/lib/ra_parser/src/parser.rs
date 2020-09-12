use crate::errors::ParserError;
use crate::block::Block;
use crate::parsed_by_token::ParsedByToken;
use ra_lexer::tokenize;
use ra_lexer::token::TokenKind;

pub fn parse<'a>(input: &'a str) -> Result<Block, (Vec<ParserError>, Option<Block>)> {
    let mut errors: Vec<ParserError> = Vec::new();
    let mut tokens_stream = tokenize(input);
    
    let mut block = {
        match Block::new_program() {
            Ok(b) => b,
            Err(e) => {
                errors.extend(e);
                return Err((errors, None));
            }
        }
    };

    while let Some(token_result) = tokens_stream.next() {
        match token_result {
            Ok(token) => {
                if token.kind == TokenKind::Comment {
                    continue;
                }
                else {
                    match block.clone().append_token(token) {
                        Ok(blk) => block=blk,
                        Err(e) => errors.extend(e)
                    }
                }
            },
            Err(e) => {
                errors.push(e.into());
            }
        }
    }

    if errors.len() == 0 {
        Ok(*block)
    }
    else {
        Err((errors, Some(*block)))
    }
}