#[macro_use] extern crate serde_derive;

pub mod block;
pub mod parser;

use ra_lexer::{RaToken, Position, TokenKind};
use block::*;

#[cfg(test)]
mod tests;
