
use crate::cursor::Position;

#[derive(Debug, PartialEq, Clone)]
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
    Plus,
    Minus,
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
    Number(char, char),
    ContentBlock(Vec<Token>),
    Undetermined
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
    pub content: String,
    pub position: (Position, Position),
    pub level: usize
}

impl Default for Token {
    fn default() -> Self {
        Self {
            len: 1,
            content: String::new(),
            position: (Position(0, 0), Position(0, 1)),
            level: 0,
            kind: TokenKind::Undetermined
        }
    }
}