#[macro_use] extern crate serde_derive;
extern crate indextree;

pub mod ast;
pub mod node;
pub mod errors;

mod block_tree;
mod expression;

use std::convert::TryInto;

use ra_lexer::{RaToken, Position, TokenKind};
use block_tree::tree::RaTree;
use serde::Serialize;
use failure::Backtrace;
use errors::ParserError;
use expression::*;


pub use ast::RaAST;
pub use node::RaASTNode;


pub fn parse<'a>(
    mut tokens_stream: impl Iterator<Item = Result<RaToken, ra_lexer::LexerError>>
) -> Result<RaAST, (Option<RaAST>, Vec<ParserError>)> {
    match block_tree::parse(tokens_stream) {
        Ok(tree) => tree.try_into(),
        Err((err, parsed)) => {
            Err((None, vec![]))
        }
    }
}

#[cfg(test)]
mod tests;