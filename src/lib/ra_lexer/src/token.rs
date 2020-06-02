
use crate::cursor::Position;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: Option<TokenKind<'a>>,
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
            kind: None
        }
    }
}