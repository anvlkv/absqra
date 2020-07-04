use crate::cursor::Position;
use std::fmt::Display;
use serde::{Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub enum TokenKind<'a> {
    Ampersand,
    Asterisk,
    At,
    CloseCurlyBrace,
    CloseParentheses,
    CloseSquareBrace,
    Colon,
    Coma,
    Comment,
    ContentBlock,
    Dollar,
    Dot,
    Equals,
    Exclamation,
    Float(f64),
    ForwardSlash,
    Greater,
    HashPound,
    Identifier(&'a str),
    Int(i64),
    Less,
    Minus,
    OpenCurlyBrace,
    OpenParentheses,
    OpenSquareBrace,
    Percent,
    Pipe,
    Plus,
    Power,
    Question,
    SemiColon,
    Slash,
    StringLiteral(&'a str),
    Tilde,
}

/// Token type
/// pub kind: Option<TokenKind<'a>>,
/// pub len: u16,
/// pub content: &'a str,
/// pub position: (Position, Position),
/// pub level: u16,
/// 
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Token<'a> {
    pub kind: Option<TokenKind<'a>>,
    pub len: u16,
    pub content: &'a str,
    pub position: (Position, Position),
    pub level: u16,
}

const EMPTY_CONTENT: &str = "";

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            len: 1,
            content: EMPTY_CONTENT,
            position: (Position(0, 0), Position(0, 1)),
            level: 0,
            kind: None,
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "token {:?}, {} at {} -> {}",
            self.kind, self.content, self.position.0, self.level
        )
    }
}
