use super::errors::LexerError;
use crate::cursor::Position;
use std::convert::TryInto;
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

// impl<'a> TryInto<RaToken> for Token<'a> {
//     type Error = LexerError;

//     fn try_into(
//         self,
//     ) -> std::result::Result<RaToken, <Self as std::convert::TryInto<RaToken>>::Error> {
//         let mut kind_value = String::new();
//         let kind = {
//             match self.kind {
//                 Some(k) => {
//                     match k {
//                         TokenKind::Identifier(id) => {
//                             kind_value = id.to_owned();
//                             TokenKind::Identifier(& kind_value)
//                         }
//                         TokenKind::StringLiteral(text) => {
//                             kind_value = text.to_owned();
//                             TokenKind::StringLiteral(& kind_value)
//                         },
//                         _ => k
//                     }
//                 },
//                 None => return Err(LexerError::UnsupportedToken(self.position.0))
//             }
//         };

//         Ok(RaToken {
//             len: self.len,
//             level: self.level,
//             position: self.position,
//             kind,
//             content: self.content.to_owned(),
            
//         })
//     }
// }
