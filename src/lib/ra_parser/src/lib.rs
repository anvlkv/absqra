#[macro_use] extern crate serde_derive;

pub mod ast;
pub mod node;

use ra_lexer::{RaToken, Position};
use ra_tree_parser::tree::RaTree;
use ast::RaAST;
use node::RaASTNode;

use serde::Serialize;
use failure::Backtrace;

#[cfg(test)]
mod tests;