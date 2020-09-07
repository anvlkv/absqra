use crate::errors::ParserError;
use crate::block::Block;
use crate::parse_by_token::ParseByToken;
use ra_lexer::tokenize;
use ra_lexer::token::TokenKind;

pub fn parse<'a>(input: &'a str) -> Result<Block, (Vec<ParserError>, Option<Block>)> {
    let mut tokens_stream = tokenize(input);
    
    let mut block: Option<Block> = None;
    let mut errors: Vec<ParserError> = Vec::new();

    while let Some(token_result) = tokens_stream.next() {
        match token_result {
            Ok(token) => {
                if token.kind == TokenKind::Comment {
                    continue;
                }
                else if block.as_ref().is_none() {
                    println!("none block {:?}", token);
                    match Block::new(token) {
                        Ok(blk) => {
                            block = Some(blk);
                        },
                        Err(e) => {
                            errors.extend(e);
                        }
                    }
                }
                else {
                    println!("some block {:?}", token);
                    match block.clone().unwrap().append_token(token) {
                        Ok(blk) => {
                            block = Some(blk);
                        }
                        Err(e) => {
                            errors.extend(e);
                        }
                    }
                }
            },
            Err(e) => {
                errors.push(e.into());
            }
        }
    }

    if errors.len() == 0 && block.is_some() {
        Ok(block.unwrap())
    }
    else {
        Err((errors, block))
    }
}