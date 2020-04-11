
use crate::cursor::Position;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind<'a> {
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
    ContentBlock([&'a Token<'a>; 16]),
    Immediate,
    Undetermined
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub len: usize,
    pub content: &'a str,
    pub position: (Position, Position),
    pub level: usize
}

const empty_content: &str = "";

impl <'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            len: 1,
            content: empty_content,
            position: (Position(0, 0), Position(0, 1)),
            level: 0,
            kind: TokenKind::Undetermined
        }
    }
}