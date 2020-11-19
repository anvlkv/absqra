#[macro_use] extern crate serde_derive;

pub mod tree;
pub mod parser;

use ra_lexer::{RaToken, Position, TokenKind};
use tree::*;

#[cfg(test)]
mod tests;
