#[macro_use] extern crate serde_derive;

mod tokenize;
pub mod errors;
pub mod position;
pub mod token;
pub mod cursor;

pub use errors::LexerError;
pub use position::Position;
pub use token::{TokenKind, RaToken};
pub use tokenize::tokenize;
use cursor::Cursor;
use serde::Serialize;
use failure::Backtrace;


#[cfg(test)]
mod tests;