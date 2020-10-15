#[macro_use] extern crate serde_derive;

pub mod errors;
pub mod position;
pub mod tokenize;
pub mod token;
pub mod cursor;

use errors::LexerError;
use position::Position;
use token::{TokenKind, RaToken};
use tokenize::tokenize;
use cursor::Cursor;
use serde::Serialize;
use failure::Backtrace;


#[cfg(test)]
mod tests;