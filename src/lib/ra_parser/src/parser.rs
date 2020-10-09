use failure::Backtrace;
use ra_lexer::cursor::Position;
use crate::errors::ParserError;
use crate::block::Block;
use crate::parsed_by_token::ParsedByToken;
use ra_lexer::tokenize;
use ra_lexer::token::TokenKind;

pub fn parse<'a>(input: &'a str) -> Result<Block, (Vec<ParserError>, Option<Block>)> {
    let mut errors: Vec<ParserError> = Vec::new();
    let mut tokens_stream = tokenize(input);
    todo!();
    // let mut block = {
    //     let mut result;
    //     match tokens_stream.next() {
    //         Some(t) => {
    //             match t {
    //                 Ok(token) => {
    //                     match Block::new_program(token) {
    //                         Ok(p) => p,
    //                         Err(e) => {
    //                             errors.extend(e);
    //                         }
    //                     }
    //                 },
    //                 Err(e) => {
    //                     errors.push(ParserError::LexerError(e, Backtrace::new()));
    //                 }
    //             }
    //         },
    //         None => {
    //             errors.push(ParserError::ExpectedAGotB("any".to_owned(), "none".to_owned(), Position::default(), Backtrace::new()));
    //         }
    //     }
    //     return Err((errors, None));
    // };

    // while let Some(token_result) = tokens_stream.next() {
    //     match token_result {
    //         Ok(token) => {
    //             if token.kind == TokenKind::Comment {
    //                 continue;
    //             }
    //             else {
    //                 match block.clone().append_token(token) {
    //                     Ok(blk) => block=blk,
    //                     Err(e) => errors.extend(e)
    //                 }
    //             }
    //         },
    //         Err(e) => {
    //             errors.push(e.into());
    //         }
    //     }
    // }

    // if errors.len() == 0 {
    //     Ok(*block)
    // }
    // else {
    //     Err((errors, Some(*block)))
    // }
}