use super::*;
use core::convert::{TryFrom, TryInto};
use cursor::{is_end_of_line};


#[derive(Serialize, Debug, Clone)]
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
}

impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool { 
        match (self, other) {
            (Self::Ampersand, Self::Ampersand) => true,
            (Self::Asterisk, Self::Asterisk) => true,
            (Self::At, Self::At) => true,
            (Self::CloseCurlyBrace, Self::CloseCurlyBrace) => true,
            (Self::CloseParentheses, Self::CloseParentheses) => true,
            (Self::CloseSquareBrace, Self::CloseSquareBrace) => true,
            (Self::Colon, Self::Colon) => true,
            (Self::Coma, Self::Coma) => true,
            (Self::Dollar, Self::Dollar) => true,
            (Self::Dot, Self::Dot) => true,
            (Self::Equals, Self::Equals) => true,
            (Self::Exclamation, Self::Exclamation) => true,
            (Self::ForwardSlash, Self::ForwardSlash) => true,
            (Self::Greater, Self::Greater) => true,
            (Self::HashPound, Self::HashPound) => true,
            (Self::Less, Self::Less) => true,
            (Self::Minus, Self::Minus) => true,
            (Self::OpenCurlyBrace, Self::OpenCurlyBrace) => true,
            (Self::OpenParentheses, Self::OpenParentheses) => true,
            (Self::OpenSquareBrace, Self::OpenSquareBrace) => true,
            (Self::Percent, Self::Percent) => true,
            (Self::Pipe, Self::Pipe) => true,
            (Self::Plus, Self::Plus) => true,
            (Self::Power, Self::Power) => true,
            (Self::Question, Self::Question) => true,
            (Self::SemiColon, Self::SemiColon) => true,
            (Self::Slash, Self::Slash) => true,
            (Self::Tilde, Self::Tilde) => true,
            (Self::Comment(a_content, a_fringed), Self::Comment(b_content, b_fringed)) => a_fringed == b_fringed && {
                a_content == b_content || a_content == "" || b_content == ""
            },
            (Self::ContentBlock(a_content), Self::ContentBlock(b_content)) => a_content == b_content || a_content == "" || b_content == "",
            (Self::FloatLiteral(a_content), Self::FloatLiteral(b_content)) => a_content == b_content || a_content == &0.0f64 || b_content == &0.0f64,
            (Self::IntegerLiteral(a_content), Self::IntegerLiteral(b_content)) => a_content == b_content || a_content == &0i64 || b_content == &0i64,
            (Self::Identifier(a_content), Self::Identifier(b_content)) => a_content == b_content || a_content == "" || b_content == "",
            (Self::StringLiteral(a_content), Self::StringLiteral(b_content)) => a_content == b_content || a_content == "" || b_content == "",
            _ => false
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct RaToken {
    pub kind: TokenKind,
    pub position: (Position, Position),
    pub level: u16,
}

impl RaToken {
    fn new_multi_char_token(cursor: &mut Cursor) -> Result<Self, LexerError> {
        assert!(cursor.ch.is_some());
        let ch = cursor.ch.unwrap();
        match ch {
            '/' => Self::new_comment(cursor),
            '`' => Self::new_content(cursor),
            '"' | '\'' => Self::new_string(cursor),
            c if c.is_numeric() 
            || c == '-'
            || c == ','
            || c == '.'  => Self::new_number(cursor),
            c if c.is_alphabetic() 
            || c == '_'=> Self::new_identifier(cursor),
            _ => Err(LexerError::UnexpectedCharacter(ch, cursor.position, Backtrace::new())),
        }
    }

    fn new_single_char_token(cursor: &mut Cursor, kind: TokenKind) -> Result<Self, LexerError> {
        let result = Ok(Self {
            kind,
            position: (
                cursor.position.clone(),
                cursor.position + Position(0, 1)
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
                    None => Err(LexerError::UnexpectedEndOfInput(start_position, Backtrace::new())),
                }
            }
            ch => Err(LexerError::UnexpectedCharacter(ch.clone(), start_position, Backtrace::new())),
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

            if content.len() == 0 && is_end_of_line(&ch) { // skip first new line
                cursor.bump();
            }
            else if ch == '`' && cursor.level == level { // close block
                end_position = Some(cursor.position.clone() + Position(0, 1));
                cursor.bump();
                let last_char = content.chars().last();
        
                if last_char.is_some() && is_end_of_line(&last_char.unwrap()) { // remove trailing new line
                    content.pop();
                }
                break;
            }
            else { // push ch
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
            None => Err(LexerError::UnexpectedEndOfInput(start_position, Backtrace::new()))
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
            }
            else {
                break;
            }
        }

        match end_position {
            Some(end_position) => Ok(Self {
                kind: TokenKind::StringLiteral(content),
                position: (start_position, end_position),
                level,
            }),
            None => Err(LexerError::UnexpectedEndOfInput(start_position, Backtrace::new()))
        }
    }

    fn new_number(cursor: &mut Cursor) -> Result<Self, LexerError> {
        let start_position = cursor.position.clone();
        let level = cursor.level.clone();
        let mut content = String::from(cursor.ch.unwrap());
        let mut parse_float = match cursor.ch.unwrap() {
            '.'
            | ',' => true,
            _ => false
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
                },
                'e' => {
                    match cursor.first_ahead() {
                        '-'
                        |'+' =>  {
                            content.push(ch);
                            cursor.bump();
                            content.push(cursor.ch.unwrap());
                            parse_float = true;
                        }
                        ch => return Err(LexerError::UnexpectedCharacter(ch, cursor.position, Backtrace::new()))
                    }
                }
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
                            Err(_) => {
                                Ok(Self {
                                    kind: TokenKind::FloatLiteral(num),
                                    position: (start_position, end_position),
                                    level,
                                })        
                            }
                        }
                    }
                    else {
                        Ok(Self {
                            kind: TokenKind::FloatLiteral(num),
                            position: (start_position, end_position),
                            level,
                        })
                    }
                },
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
                '-'
                |'.'
                |',' => match cursor.first_ahead() {
                    c if c.is_numeric() => Self::new_multi_char_token(cursor),
                    _ => {
                        match ch {
                            '-' => Self::new_single_char_token(cursor, TokenKind::Minus),
                            ',' => Self::new_single_char_token(cursor, TokenKind::Coma),
                            '.' => Self::new_single_char_token(cursor, TokenKind::Dot),
                            _ => panic!("what have you done!?")
                        }
                    }
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
            None => Err(LexerError::UnexpectedEndOfInput(cursor.position, Backtrace::new())),
        }
    }
}


