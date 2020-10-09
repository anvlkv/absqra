use super::*;
use core::convert::TryFrom;

#[derive(Serialize, Debug)]
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
    Float(f64),
    ForwardSlash,
    Greater,
    HashPound,
    Identifier(String),
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
    StringLiteral(String),
    Tilde,
}

#[derive(Serialize, Debug)]
pub struct RaToken {
    kind: TokenKind,
    position: (Position, Position),
    level: u16,
}

impl RaToken {
    fn new_multi_char_token(cursor: &mut Cursor) -> Result<Self, LexerError> {
        assert!(cursor.ch.is_some());
        let ch = cursor.ch.unwrap();
        match ch {
            '/' => Self::new_comment(cursor),
            '`' => Self::new_content(cursor),
            '"' | '\'' => Self::new_string(cursor),
            c if c.is_numeric() => Self::new_number(cursor),
            c if c.is_alphabetic() || c == '_' => Self::new_identifier(cursor),
            _ => Err(LexerError::UnexpectedCharacter(ch, cursor.position)),
        }
    }

    fn new_single_char_token(cursor: &mut Cursor, kind: TokenKind) -> Result<Self, LexerError> {
        let result = Ok(Self {
            kind,
            position: (
                cursor.position.clone(),
                Position(cursor.position.0, cursor.position.1 + 1),
            ),
            level: cursor.level,
        });

        cursor.bump();

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
                fringed = { cursor.ch.is_some() && cursor.ch.unwrap() == '/' };

                while cursor.position.0 == start_position.0 && cursor.ch.is_some() {
                    content.push(cursor.ch.unwrap());
                    cursor.bump();
                }

                Ok(Self {
                    kind: TokenKind::Comment(content, fringed),
                    position: (start_position, cursor.position.clone()),
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
                                end_position = Some(cursor.position.clone());
                                cursor.bump();
                                break;
                            }
                            ch => {
                                content.push(ch);
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
                    None => Err(LexerError::UnexpectedEndOfInput(cursor.position)),
                }
            }
            ch => Err(LexerError::UnexpectedCharacter(ch, cursor.position)),
        }
    }

    fn new_content(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
        let level = cursor.level.clone();
        let mut content = String::new();
        let mut end_position = None;

        cursor.bump();
        while cursor.ch.is_some() {
            if cursor.ch.unwrap() == '`' && cursor.level == level {
                end_position = Some(cursor.position.clone());
                cursor.bump();
                break;
            } else {
                content.push(cursor.ch.unwrap());
                cursor.bump();
            }
        }

        match end_position {
            Some(end_position) => Ok(Self {
                kind: TokenKind::ContentBlock(content),
                position: (start_position, end_position),
                level,
            }),
            None => Err(LexerError::UnexpectedEndOfInput(cursor.position))
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
                end_position = Some(cursor.position.clone());
                cursor.bump();
                break;
            } else {
                content.push(cursor.ch.unwrap());
                cursor.bump();
            }
        }

        match end_position {
            Some(end_position) => Ok(Self {
                kind: TokenKind::StringLiteral(content),
                position: (start_position, end_position),
                level,
            }),
            None => Err(LexerError::UnexpectedEndOfInput(cursor.position))
        }
    }

    fn new_number(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
        let level = cursor.level.clone();
        let mut content = String::from(cursor.ch.unwrap());
        let mut has_decimal_separator = false;

        cursor.bump();

        while cursor.ch.is_some() {
            match cursor.ch.unwrap() {
                ch if ch.is_numeric() => content.push(cursor.ch.unwrap()),
                ',' | '.' => {
                    has_decimal_separator = true;
                    content.push('.');
                }
                _ => break,
            }
            cursor.bump();
        }

        if has_decimal_separator {
            match content.parse::<f64>() {
                Ok(num) => Ok(Self {
                    kind: TokenKind::Float(num),
                    position: (start_position, cursor.position.clone()),
                    level,
                }),
                Err(e) => Err(LexerError::InvalidFloat(e, cursor.position)),
            }
        } else {
            match content.parse::<i64>() {
                Ok(num) => Ok(Self {
                    kind: TokenKind::Int(num),
                    position: (start_position, cursor.position.clone()),
                    level,
                }),
                Err(e) => Err(LexerError::InvalidInt(e, cursor.position)),
            }
        }
    }

    fn new_identifier(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
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
            cursor.bump();
        }

        Ok(Self {
            kind: TokenKind::Identifier(content),
            position: (start_position, cursor.position.clone()),
            level,
        })
    }
}

impl TryFrom<&mut Cursor> for RaToken {
    type Error = LexerError;
    fn try_from(cursor: &mut Cursor) -> Result<Self, Self::Error> {
        match cursor.ch {
            Some(ch) => match ch {
                '/' => match cursor.first_ahead() {
                    '/' | '*' => Self::new_multi_char_token(cursor),
                    _ => Self::new_single_char_token(cursor, TokenKind::Slash),
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
                ',' => Self::new_single_char_token(cursor, TokenKind::Coma),
                '.' => Self::new_single_char_token(cursor, TokenKind::Dot),
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
                '-' => match cursor.first_ahead() {
                    c if c.is_numeric() => Self::new_multi_char_token(cursor),
                    _ => Self::new_single_char_token(cursor, TokenKind::Minus),
                },
                _ => Self::new_multi_char_token(cursor),
            },
            None => Err(LexerError::UnexpectedEndOfInput(cursor.position)),
        }
    }
}
