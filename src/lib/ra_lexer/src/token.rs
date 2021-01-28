//! token

use super::*;
use core::convert::{TryFrom, TryInto};
use cursor::is_end_of_line;

/// Single character tokens, parsed comments, string, numbers, content tokens
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ampersand,
    Asterisk,
    At,
    CloseCurlyBrace,
    CloseParentheses,
    CloseSquareBrace,
    Colon,
    Coma,
    Comment(String, bool),
    ContentBlock(String),
    Dollar,
    Dot,
    Equals,
    Exclamation,
    FloatLiteral(f64),
    ForwardSlash,
    Greater,
    HashPound,
    Identifier(String),
    IntegerLiteral(i64),
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
    StringLiteral(String),
    Tilde,
    NONE,
}

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::NONE
    }
}

impl TokenKind {
    pub fn variant_eq(&self, b: &TokenKind) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(b)
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct RaToken {
    pub kind: TokenKind,
    pub position: (Position, Position),
    pub level: u16,
}

impl PartialEq for RaToken {
    fn eq(&self, other: &Self) -> bool {
        (self.kind == TokenKind::default() || other.kind == TokenKind::default())
            || (self.kind == other.kind
                && self.position == other.position
                && self.level == self.level)
    }
}

impl RaToken {
    pub fn closing_pair(&self) -> Option<TokenKind> {
        match self.kind {
            TokenKind::OpenCurlyBrace => Some(TokenKind::CloseCurlyBrace),
            TokenKind::OpenSquareBrace => Some(TokenKind::CloseSquareBrace),
            TokenKind::OpenParentheses => Some(TokenKind::CloseParentheses),
            _ => None,
        }
    }

    fn new_multi_char_token(cursor: &mut Cursor) -> Result<Self, LexerError> {
        assert!(cursor.ch.is_some());
        let ch = cursor.ch.unwrap();
        let result = match ch {
            '/' => Self::new_comment(cursor),
            '`' => Self::new_content(cursor),
            '"' | '\'' => Self::new_string(cursor),
            c if c.is_numeric() || c == '-' || c == ',' || c == '.' => Self::new_number(cursor),
            c if c.is_alphabetic() || c == '_' => Self::new_identifier(cursor),
            _ => Err(LexerError::UnexpectedCharacter(
                ch,
                cursor.position,
                Backtrace::new(),
            )),
        };

        cursor.last_parsed = match &result {
            Ok(p) => Some(p.clone()),
            Err(_) => None,
        };

        result
    }

    fn new_single_char_token(cursor: &mut Cursor, kind: TokenKind) -> Result<Self, LexerError> {
        let result = Ok(Self {
            kind,
            position: (cursor.position.clone(), cursor.position + Position(0, 1)),
            level: cursor.level,
        });

        cursor.bump();

        cursor.last_parsed = match &result {
            Ok(p) => Some(p.clone()),
            Err(_) => None,
        };

        result
    }

    fn new_comment(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
        let level = cursor.level.clone();
        let mut content = String::new();
        let mut fringed = true;

        match cursor.first_ahead() {
            '/' => {
                cursor.bump();
                cursor.bump();
                fringed = {
                    let fringed = cursor.ch.is_some() && cursor.ch.unwrap() == '/';

                    if fringed {
                        cursor.bump();
                    }

                    fringed
                };

                let mut end_position = Position(0, 0);
                while cursor.position.0 == start_position.0 && cursor.ch.is_some() {
                    if !is_end_of_line(&cursor.ch.unwrap()) {
                        content.push(cursor.ch.unwrap());
                        end_position = cursor.position + Position(0, 1);
                    }
                    cursor.bump();
                }

                Ok(Self {
                    kind: TokenKind::Comment(content, fringed),
                    position: (start_position, end_position),
                    level,
                })
            }
            '*' => {
                cursor.bump();
                cursor.bump();

                let mut end_position = None;

                while cursor.ch.is_some() {
                    fringed = {
                        let result =
                            fringed && (!cursor.is_line_start || cursor.ch.unwrap() == '*');
                        if fringed && cursor.is_line_start && cursor.ch.unwrap() == '*' {
                            cursor.bump();
                        }
                        result
                    };

                    match cursor.ch.unwrap() {
                        '*' => match cursor.first_ahead() {
                            '/' => {
                                cursor.bump();
                                end_position = Some(cursor.position + Position(0, 1));
                                cursor.bump();
                                break;
                            }
                            _ => {
                                content.push('*');
                                cursor.bump();
                            }
                        },
                        ch => {
                            content.push(ch);
                            cursor.bump();
                        }
                    }
                }

                match end_position {
                    Some(end_position) => Ok(Self {
                        kind: TokenKind::Comment(content, fringed),
                        position: (start_position, end_position),
                        level,
                    }),
                    None => Err(LexerError::UnexpectedEndOfInput(
                        start_position,
                        Backtrace::new(),
                    )),
                }
            }
            ch => Err(LexerError::UnexpectedCharacter(
                ch.clone(),
                start_position,
                Backtrace::new(),
            )),
        }
    }

    fn new_content(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
        let level = cursor.level.clone();
        let mut content = String::new();
        let mut end_position = None;

        cursor.ruler = Some(level + 1);

        cursor.bump();
        while cursor.ch.is_some() {
            let ch = cursor.ch.unwrap();

            if content.len() == 0 && is_end_of_line(&ch) {
                // skip first new line
                cursor.bump();
            } else if ch == '`' && cursor.level == level {
                // close block
                end_position = Some(cursor.position.clone() + Position(0, 1));
                cursor.bump();
                let last_char = content.chars().last();

                if last_char.is_some() && is_end_of_line(&last_char.unwrap()) {
                    // remove trailing new line
                    content.pop();
                }
                break;
            } else {
                // push ch
                content.push(ch);
                cursor.bump();
            }
        }

        cursor.ruler = None;

        match end_position {
            Some(end_position) => Ok(Self {
                kind: TokenKind::ContentBlock(content),
                position: (start_position, end_position),
                level,
            }),
            None => Err(LexerError::UnexpectedEndOfInput(
                start_position,
                Backtrace::new(),
            )),
        }
    }

    fn new_string(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
        let level = cursor.level.clone();
        let opening_char = cursor.ch.unwrap();
        let mut end_position = None;
        let mut content = String::new();
        cursor.bump();

        while cursor.ch.is_some() {
            if cursor.ch.unwrap() == '\\' && cursor.first_ahead() == opening_char {
                cursor.bump();
                content.push(cursor.ch.unwrap());
            } else if cursor.ch.unwrap() == opening_char {
                end_position = Some(cursor.position + Position(0, 1));
                cursor.bump();
                break;
            } else if !is_end_of_line(&cursor.ch.unwrap()) {
                content.push(cursor.ch.unwrap());
                cursor.bump();
            } else {
                break;
            }
        }

        match end_position {
            Some(end_position) => Ok(Self {
                kind: TokenKind::StringLiteral(content),
                position: (start_position, end_position),
                level,
            }),
            None => Err(LexerError::UnexpectedEndOfInput(
                start_position,
                Backtrace::new(),
            )),
        }
    }

    fn new_number(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
        let level = cursor.level.clone();
        let mut content = String::from(cursor.ch.unwrap());
        let mut parse_float = match cursor.ch.unwrap() {
            '.' | ',' => true,
            _ => false,
        };
        let mut end_position = cursor.position.clone();
        cursor.bump();

        while cursor.ch.is_some() {
            let ch = cursor.ch.unwrap();
            match ch {
                c if c.is_numeric() => content.push(ch),
                ',' | '.' => {
                    parse_float = true;
                    content.push('.');
                }
                'e' => match cursor.first_ahead() {
                    '-' | '+' => {
                        content.push(ch);
                        cursor.bump();
                        content.push(cursor.ch.unwrap());
                        parse_float = true;
                    }
                    ch => {
                        return Err(LexerError::UnexpectedCharacter(
                            ch,
                            cursor.position,
                            Backtrace::new(),
                        ))
                    }
                },
                _ => break,
            }
            end_position = cursor.position.clone();
            cursor.bump();
        }

        end_position = end_position + Position(0, 1);

        if parse_float {
            match content.parse::<f64>() {
                Ok(num) => {
                    if num % 1.0 == 0.0 {
                        match format!("{}", num).parse::<i64>() {
                            Ok(num) => Ok(Self {
                                kind: TokenKind::IntegerLiteral(num),
                                position: (start_position, end_position),
                                level,
                            }),
                            Err(_) => Ok(Self {
                                kind: TokenKind::FloatLiteral(num),
                                position: (start_position, end_position),
                                level,
                            }),
                        }
                    } else {
                        Ok(Self {
                            kind: TokenKind::FloatLiteral(num),
                            position: (start_position, end_position),
                            level,
                        })
                    }
                }
                Err(e) => Err(LexerError::InvalidFloat(e, start_position)),
            }
        } else {
            match content.parse::<i64>() {
                Ok(num) => Ok(Self {
                    kind: TokenKind::IntegerLiteral(num),
                    position: (start_position, end_position),
                    level,
                }),
                Err(e) => Err(LexerError::InvalidInt(e, start_position)),
            }
        }
    }

    fn new_identifier(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
        let mut end_position = Position(0, 0);
        let level = cursor.level.clone();
        let mut content = String::from(cursor.ch.unwrap());
        cursor.bump();

        while cursor.ch.is_some() {
            match cursor.ch.unwrap() {
                ch if ch.is_alphanumeric() => content.push(cursor.ch.unwrap()),
                '_' => {
                    content.push('_');
                }
                _ => break,
            }
            end_position = cursor.position;
            cursor.bump();
        }

        end_position = end_position + Position(0, 1);

        Ok(Self {
            kind: TokenKind::Identifier(content),
            position: (start_position, end_position),
            level,
        })
    }
}

impl<'c> TryFrom<&mut Cursor<'c>> for RaToken {
    type Error = LexerError;
    fn try_from(cursor: &mut Cursor) -> Result<Self, Self::Error> {
        match cursor.ch {
            Some(ch) => match ch {
                '/' => match cursor.first_ahead() {
                    '/' | '*' => Self::new_multi_char_token(cursor),
                    _ => Self::new_single_char_token(cursor, TokenKind::Slash),
                },
                '-' | '.' | ',' => match cursor.first_ahead() {
                    c if c.is_numeric() => {
                        if ch == '-'
                            && cursor.last_parsed.is_some()
                            && (cursor.last_parsed.as_ref().unwrap().position.0).0
                                == cursor.position.0
                            && (cursor
                                .last_parsed
                                .as_ref()
                                .unwrap()
                                .kind
                                .variant_eq(&TokenKind::FloatLiteral(Default::default()))
                                || cursor
                                    .last_parsed
                                    .as_ref()
                                    .unwrap()
                                    .kind
                                    .variant_eq(&TokenKind::IntegerLiteral(Default::default())))
                        {
                            Self::new_single_char_token(cursor, TokenKind::Minus)
                        } else {
                            Self::new_multi_char_token(cursor)
                        }
                    }
                    _ => match ch {
                        '-' => Self::new_single_char_token(cursor, TokenKind::Minus),
                        ',' => Self::new_single_char_token(cursor, TokenKind::Coma),
                        '.' => Self::new_single_char_token(cursor, TokenKind::Dot),
                        c => Err(LexerError::UnexpectedCharacter(
                            c,
                            cursor.position,
                            Backtrace::new(),
                        )),
                    },
                },
                '*' => Self::new_single_char_token(cursor, TokenKind::Asterisk),
                '>' => Self::new_single_char_token(cursor, TokenKind::Greater),
                '<' => Self::new_single_char_token(cursor, TokenKind::Less),
                '!' => Self::new_single_char_token(cursor, TokenKind::Exclamation),
                '?' => Self::new_single_char_token(cursor, TokenKind::Question),
                '{' => Self::new_single_char_token(cursor, TokenKind::OpenCurlyBrace),
                '[' => Self::new_single_char_token(cursor, TokenKind::OpenSquareBrace),
                '(' => Self::new_single_char_token(cursor, TokenKind::OpenParentheses),
                '}' => Self::new_single_char_token(cursor, TokenKind::CloseCurlyBrace),
                ']' => Self::new_single_char_token(cursor, TokenKind::CloseSquareBrace),
                ')' => Self::new_single_char_token(cursor, TokenKind::CloseParentheses),
                ':' => Self::new_single_char_token(cursor, TokenKind::Colon),
                '+' => Self::new_single_char_token(cursor, TokenKind::Plus),
                '=' => Self::new_single_char_token(cursor, TokenKind::Equals),
                ';' => Self::new_single_char_token(cursor, TokenKind::SemiColon),
                '&' => Self::new_single_char_token(cursor, TokenKind::Ampersand),
                '#' => Self::new_single_char_token(cursor, TokenKind::HashPound),
                '@' => Self::new_single_char_token(cursor, TokenKind::At),
                '\\' => Self::new_single_char_token(cursor, TokenKind::ForwardSlash),
                '|' => Self::new_single_char_token(cursor, TokenKind::Pipe),
                '%' => Self::new_single_char_token(cursor, TokenKind::Percent),
                '$' => Self::new_single_char_token(cursor, TokenKind::Dollar),
                '^' => Self::new_single_char_token(cursor, TokenKind::Power),
                '~' => Self::new_single_char_token(cursor, TokenKind::Tilde),
                _ => Self::new_multi_char_token(cursor),
            },
            None => Err(LexerError::UnexpectedEndOfInput(
                cursor.position,
                Backtrace::new(),
            )),
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ampersand => write!(f, "&"),
            Self::Asterisk => write!(f, "*"),
            Self::At => write!(f, "@"),
            Self::CloseCurlyBrace => write!(f, "}}"),
            Self::CloseParentheses => write!(f, ")"),
            Self::CloseSquareBrace => write!(f, "]"),
            Self::Colon => write!(f, ":"),
            Self::Coma => write!(f, ","),
            Self::Comment(content, fringe) => write!(f, "{} {}", fringe, content),
            Self::ContentBlock(content) => write!(f, "`{}`", content),
            Self::Dollar => write!(f, "$"),
            Self::Dot => write!(f, "."),
            Self::Equals => write!(f, "="),
            Self::Exclamation => write!(f, "!"),
            Self::FloatLiteral(num) => write!(f, "{}", num),
            Self::ForwardSlash => write!(f, "\\"),
            Self::Greater => write!(f, ">"),
            Self::HashPound => write!(f, "#"),
            Self::Identifier(id) => write!(f, "{}", id),
            Self::IntegerLiteral(num) => write!(f, "{}", num),
            Self::Less => write!(f, "<"),
            Self::Minus => write!(f, "-"),
            Self::OpenCurlyBrace => write!(f, "{{"),
            Self::OpenParentheses => write!(f, "("),
            Self::OpenSquareBrace => write!(f, "["),
            Self::Percent => write!(f, "%"),
            Self::Pipe => write!(f, "|"),
            Self::Plus => write!(f, "+"),
            Self::Power => write!(f, "^"),
            Self::Question => write!(f, "?"),
            Self::SemiColon => write!(f, ";"),
            Self::Slash => write!(f, "/"),
            Self::StringLiteral(content) => write!(f, "{}", content),
            Self::Tilde => write!(f, "~"),
            Self::NONE => write!(f, ""),
        }
    }
}
