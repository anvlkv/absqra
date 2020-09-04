use crate::cursor::Position;
use crate::LexerError;
use serde::Serialize;
use std::convert::TryFrom;
use std::fmt::Display;

const EMPTY_CONTENT: &str = "";

#[derive(Debug, Clone, Copy, Serialize)]
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

impl<'a> PartialEq for TokenKind<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&TokenKind::Identifier(_), &TokenKind::Identifier(_)) => true,
            (&TokenKind::StringLiteral(_), &TokenKind::StringLiteral(_)) => true,
            (&TokenKind::Int(_), &TokenKind::Int(_)) => true,
            (&TokenKind::Float(_), &TokenKind::Float(_)) => true,
            (&TokenKind::Ampersand, &TokenKind::Ampersand) => true,
            (&TokenKind::Asterisk, &TokenKind::Asterisk) => true,
            (&TokenKind::At, &TokenKind::At) => true,
            (&TokenKind::CloseCurlyBrace, &TokenKind::CloseCurlyBrace) => true,
            (&TokenKind::CloseParentheses, &TokenKind::CloseParentheses) => true,
            (&TokenKind::CloseSquareBrace, &TokenKind::CloseSquareBrace) => true,
            (&TokenKind::Colon, &TokenKind::Colon) => true,
            (&TokenKind::Coma, &TokenKind::Coma) => true,
            (&TokenKind::Comment, &TokenKind::Comment) => true,
            (&TokenKind::ContentBlock, &TokenKind::ContentBlock) => true,
            (&TokenKind::Dollar, &TokenKind::Dollar) => true,
            (&TokenKind::Dot, &TokenKind::Dot) => true,
            (&TokenKind::Equals, &TokenKind::Equals) => true,
            (&TokenKind::Exclamation, &TokenKind::Exclamation) => true,
            (&TokenKind::ForwardSlash, &TokenKind::ForwardSlash) => true,
            (&TokenKind::Greater, &TokenKind::Greater) => true,
            (&TokenKind::HashPound, &TokenKind::HashPound) => true,
            (&TokenKind::Less, &TokenKind::Less) => true,
            (&TokenKind::Minus, &TokenKind::Minus) => true,
            (&TokenKind::OpenCurlyBrace, &TokenKind::OpenCurlyBrace) => true,
            (&TokenKind::OpenParentheses, &TokenKind::OpenParentheses) => true,
            (&TokenKind::OpenSquareBrace, &TokenKind::OpenSquareBrace) => true,
            (&TokenKind::Percent, &TokenKind::Percent) => true,
            (&TokenKind::Pipe, &TokenKind::Pipe) => true,
            (&TokenKind::Plus, &TokenKind::Plus) => true,
            (&TokenKind::Power, &TokenKind::Power) => true,
            (&TokenKind::Question, &TokenKind::Question) => true,
            (&TokenKind::SemiColon, &TokenKind::SemiColon) => true,
            (&TokenKind::Slash, &TokenKind::Slash) => true,
            (&TokenKind::Tilde, &TokenKind::Tilde) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub(crate) struct Token<'a> {
    pub kind: Option<TokenKind<'a>>,
    pub len: u16,
    pub content: &'a str,
    pub position: (Position, Position),
    pub level: u16,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct RaToken<'a> {
    pub kind: TokenKind<'a>,
    pub len: u16,
    pub content: &'a str,
    pub position: (Position, Position),
    pub level: u16,
}

impl<'a> TryFrom<Token<'a>> for RaToken<'a> {
    type Error = LexerError;

    fn try_from(token: Token<'a>) -> Result<Self, <Self as TryFrom<Token<'a>>>::Error> {
        if token.kind.is_none() {
            Err(LexerError::UnsupportedToken(token.position.0))
        } else {
            Ok(RaToken {
                kind: token.kind.unwrap(),
                level: token.level,
                position: token.position,
                content: token.content,
                len: token.len,
            })
        }
    }
}

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
