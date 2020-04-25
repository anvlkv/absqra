
use crate::cursor::Position;
// use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    SemiColon,
    Coma,
    Dot,
    Colon,
    Greater,
    GreaterOrEquals,
    Less,
    LessOrEquals,
    Equals,
    NotEquals,
    Plus,
    Minus,
    AddAssign,
    SubtractAssign,
    Exclamation,
    Slash,
    Comment,
    ForwardSlash,
    Ampersand,
    Pipe,
    OpenCurlyBrace,
    OpenSquareBrace,
    OpenParentheses,
    CloseCurlyBrace,
    CloseSquareBrace,
    CloseParentheses,
    At,
    HashPound,
    Asterisk,
    Percent,
    Dollar,
    Power,
    Tilde,
    Question,
    Identifier,
    StringLiteral,
    Int(i64),
    Float(f64),
    ContentBlock,
    Immediate,
    Undetermined
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub len: u16,
    pub content: &'a str,
    pub position: (Position, Position),
    pub level: u16
}

const EMPTY_CONTENT: &str = "";

impl <'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            len: 1,
            content: EMPTY_CONTENT,
            position: (Position(0, 0), Position(0, 1)),
            level: 0,
            kind: TokenKind::Undetermined
        }
    }
}