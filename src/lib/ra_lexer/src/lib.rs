#[macro_use] extern crate failure_derive;
#[macro_use] extern crate serde_derive;

extern crate serde;

pub mod cursor;
pub mod errors;
pub mod token;

use token::{Token, TokenKind};
use cursor::{Cursor, Position, is_end_of_line, is_whitespace, EOF_CHAR};
use errors::LexerError;
use std::convert::TryInto;

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Token<'a>> + 'a {
    tokenize_cursor(Cursor::new(input, Position(1, 0), 0, 0))
}

fn tokenize_cursor<'a>(mut cursor: Cursor<'a>) -> impl Iterator<Item = Token<'a>> + 'a {
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            return None;
        }
        match cursor.advance_token() {
            Ok(t) => {
                match t.kind {
                    Some(_) => Some(t),
                    None => {
                        panic!("{:?}", LexerError::UnsupportedToken(cursor.position))
                    }
                }
            },
            Err(e) => {
                println!("{:?}", cursor.position);
                panic!("{:?}", e);
            }
        }
    })
}


impl <'a> Cursor<'a> {
    fn advance_token(&mut self) -> Result<Token<'a>, LexerError> {
        let mut start_position = self.position.clone();
        let first_char = match self.bump() {
            Some(ch) => ch,
            None => return Err(LexerError::UnexpectedEndOfInput(self.position))
        };
 
        if self.position.0 > start_position.0 {
            start_position = Position(self.position.0, self.position.1 - 1); // adjust position for after line change
        }

        let start_consumed = self.len_consumed() - 1;


        match first_char {
            '/' => match self.first_ahead() {
                '/' => self.single_line_comment(start_position.clone()),
                '*' => self.multi_line_comment(start_position.clone()),
                _ => self.single_character_token(TokenKind::Slash, start_position, start_consumed),
            },
            '>' => self.single_character_token(TokenKind::Greater, start_position, start_consumed),
            '<' => self.single_character_token(TokenKind::Less, start_position, start_consumed),
            '!' => self.single_character_token(TokenKind::Exclamation, start_position, start_consumed),
            '?' => self.single_character_token(TokenKind::Question, start_position, start_consumed),
            '{' => self.single_character_token(TokenKind::OpenCurlyBrace, start_position, start_consumed),
            '[' => self.single_character_token(TokenKind::OpenSquareBrace, start_position, start_consumed),
            '(' => self.single_character_token(TokenKind::OpenParentheses, start_position, start_consumed),
            '}' => self.single_character_token(TokenKind::CloseCurlyBrace, start_position, start_consumed),
            ']' => self.single_character_token(TokenKind::CloseSquareBrace, start_position, start_consumed),
            ')' => self.single_character_token(TokenKind::CloseParentheses, start_position, start_consumed),
            ':' => self.single_character_token(TokenKind::Colon, start_position, start_consumed),
            ',' => self.single_character_token(TokenKind::Coma, start_position, start_consumed),
            '.' => self.single_character_token(TokenKind::Dot, start_position, start_consumed),
            '+' => self.single_character_token(TokenKind::Plus, start_position, start_consumed),
            '=' => self.single_character_token(TokenKind::Equals, start_position, start_consumed),
            ';' => self.single_character_token(TokenKind::SemiColon, start_position, start_consumed),
            '&' => self.single_character_token(TokenKind::Ampersand, start_position, start_consumed),
            '#' => self.single_character_token(TokenKind::HashPound, start_position, start_consumed),
            '@' => self.single_character_token(TokenKind::At, start_position, start_consumed),
            '\\' => self.single_character_token(TokenKind::ForwardSlash, start_position, start_consumed),
            '|' => self.single_character_token(TokenKind::Pipe, start_position, start_consumed),
            '%' => self.single_character_token(TokenKind::Percent, start_position, start_consumed),
            '$' => self.single_character_token(TokenKind::Dollar, start_position, start_consumed),
            '^' => self.single_character_token(TokenKind::Power, start_position, start_consumed),
            '~' => self.single_character_token(TokenKind::Tilde, start_position, start_consumed),
            '`' => self.content_block(start_position.clone()),
            '-' => {
                match self.first_ahead() {
                    c if c.is_numeric() => self.number(start_position),
                    _ => self.single_character_token(TokenKind::Minus, start_position, start_consumed)
                }
            },
            c if c.is_alphabetic() || c == '_' => self.identifier(start_position),
            c if c.is_numeric() => self.number(start_position),
            c if c == '\'' => self.string_literal(c, start_position),
            c if c == '"' => self.string_literal(c, start_position),
            c if is_whitespace(&c) || is_end_of_line(&c) => self.advance_token(),
            c => Err(LexerError::UnexpectedCharacter(c.clone(), start_position))
        }
    }

    fn single_character_token(&mut self, kind: TokenKind<'a>, start_position: Position, start_consumed: usize) -> Result<Token<'a>, LexerError> {
        Ok(Token {
            kind: Some(kind),
            position: (start_position, self.position.clone()),
            content: self.slice(start_consumed, self.len_consumed()),
            level: self.level.clone(),
            len: 1,
            ..Default::default()
        })
    }

    fn string_literal(&mut self, opening_quote: char, start_position: Position) -> Result<Token<'a>, LexerError> {
        let mut string_literal = Token {
            ..Default::default()
        };

        let start_consumed = self.len_consumed() - 1;

        loop {
            if self.position.0 > start_position.0 {
                return Err(LexerError::UnexpectedEndOfLine(self.position));
            }

            match self.bump() {
                Some(c) => {
                    match c {
                        ch if ch == opening_quote => break,
                        _ => {}
                    }
                }
                None => return Err(LexerError::UnexpectedEndOfInput(self.position))
            }
        }

        string_literal.level = self.level.clone();
        string_literal.position = (start_position, self.position.clone());
        string_literal.len = (self.len_consumed() - start_consumed).try_into().unwrap();
        string_literal.content = self.slice(start_consumed + 1, self.len_consumed() - 1);
        string_literal.kind = Some(TokenKind::StringLiteral(string_literal.content));

        Ok(string_literal)
    }

    fn number(&mut self, start_position: Position) -> Result<Token<'a>, LexerError> {
        let mut number = Token {
            ..Default::default()
        };

        let mut first_separator: char = ' ';
        let mut second_separator: char = ' ';
        let start_consumed = self.len_consumed() - 1; // add 1 for first token

        loop {
            let next_character = self.first_ahead();
            match next_character {
                c if is_end_of_line(&c) => break,
                EOF_CHAR => break,
                '.'|',' => {
                    if !self.second_ahead().is_numeric() {
                        break;
                    }
                    if first_separator == ' ' {
                        first_separator = next_character;
                    }
                    else if first_separator != next_character && second_separator == ' ' {
                        second_separator = next_character;
                    }
                    else if second_separator == next_character && first_separator != ' ' {
                        return Err(LexerError::UnexpectedCharacter(next_character, self.position))
                    }
                    self.bump();
                }
                c if c.is_numeric() => {self.bump();},
                _ => break
            }
        }

        number.len = (self.len_consumed() - start_consumed).try_into().unwrap();
        number.position = (start_position, self.position.clone());
        number.level = self.level.clone();
        number.content = self.slice(start_consumed, self.len_consumed());

        number.kind = {
            let thousands_separator = match second_separator {
                ',' => '.',
                '.' => ',',
                _ => ' '
            };

            let decimals_separator = match second_separator {
                ',' => ',',
                '.' => '.',
                _ => first_separator
            };

            let mut num = number.content.to_owned();

            num = num.replace(thousands_separator, "");

            if let Some(_) = num.find(decimals_separator) {
                num = num.replace(decimals_separator, ".");
                match num.parse::<f64>() {
                    Ok(n) => Some(TokenKind::Float(n)),
                    Err(e) => return Err(LexerError::InvalidFloat(e, self.position))
                }
            }
            else {
                match num.parse::<i64>() {
                    Ok(n) => Some(TokenKind::Int(n)),
                    Err(e) => return Err(LexerError::InvalidInt(e, self.position))
                }
            }
        };

        Ok(number)
    }

    fn identifier(&mut self, start_position: Position) -> Result<Token<'a>, LexerError> {
        let mut identifier = Token {
            ..Default::default()
        };
        let start_consumed = self.len_consumed() - 1; // add 1 for first token

        loop {
            let next_character = self.first_ahead();
            match next_character {
                c if c.is_alphabetic() || c.is_numeric() || c == '_' => {
                    self.bump();
                },
                _ => break
            }
        }

        identifier.position = (start_position, self.position.clone());
        identifier.len = (self.len_consumed() - start_consumed).try_into().unwrap();
        identifier.level = self.level.clone();
        identifier.content = self.slice(start_consumed, self.len_consumed());
        identifier.kind = Some(TokenKind::Identifier(identifier.content));

        Ok(identifier)
    }

    fn content_block(&mut self, start_position: Position) -> Result<Token<'a>, LexerError> {
        let mut content_block = Token{
            kind: Some(TokenKind::ContentBlock),
            ..Default::default()
        };
        let initial_level = self.level;
        let start_consumed = self.len_consumed() - 1; // add 1 for first token
        let mut block_closed = false;
        while let Some(ch) = self.bump() {
            if self.level < initial_level {
                if !is_end_of_line(&ch) {
                    return Err(LexerError::UnexpectedIndentLevel(self.level, self.position));
                }
            }
            else if self.level == initial_level && ch == '`' {
                block_closed = true;
                break;
            }
        }

        if !block_closed {
            return Err(LexerError::UnexpectedEndOfInput(self.position));
        }

        content_block.content = self.slice(start_consumed + 1, self.len_consumed() - 1);
        content_block.position = (start_position, self.position.clone());
        content_block.len = (self.len_consumed() - start_consumed).try_into().unwrap();
        content_block.level = self.level.clone();

        Ok(content_block)
    }

    fn single_line_comment (&mut self, start_position: Position) -> Result<Token<'a>, LexerError> {
        self.bump();
        let mut comment = Token{
            kind: Some(TokenKind::Comment),
            ..Default::default()
        };

        let start_consumed = self.len_consumed();

        while let Some(_) = self.bump() {
            match self.first_ahead() {
                c if is_end_of_line(&c) => break,
                EOF_CHAR => break,
                _ => {}
            }
        };

        comment.position = (start_position, self.position.clone());
        comment.len = (self.len_consumed() - start_consumed + 2).try_into().unwrap(); // add 2 for "//"
        comment.level = self.level.clone();
        comment.content = self.slice(start_consumed, self.len_consumed());
        
        Ok(comment)
    }

    fn multi_line_comment (&mut self, start_position: Position) -> Result<Token<'a>, LexerError> {
        self.bump();
        let mut comment = Token{
            kind: Some(TokenKind::Comment),
            level: self.level.clone(),
            ..Default::default()
        };

        let start_consumed = self.len_consumed();
        while let Some(ch) = self.bump() {

            match ch {
                '*' => match self.first_ahead() {
                    '/' => {
                        self.bump();
                        break;
                    },
                    _ => {
                        self.bump();
                    }
                },
                _ => {
                    self.bump();
                }
            }
        };

        comment.position = (start_position, self.position.clone());
        comment.len = (self.len_consumed() - start_consumed + 2).try_into().unwrap(); // add 2 for "/*"
        comment.content = self.slice(start_consumed, self.len_consumed() - 2);

        Ok(comment)
    }
}

#[cfg(test)]
mod tests;
