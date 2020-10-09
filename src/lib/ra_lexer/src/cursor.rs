use core::iter::Peekable;
use std::str::Chars;
use super::*;
// use std::str::Chars;
use std::convert::{TryInto, TryFrom};

pub (crate) struct Cursor<'char> {
    pub position: Position,
    pub level: u16,
    pub ch: Option<char>,
    pub is_line_start: bool,
    indent_width: u8,
    chars: Peekable<Chars<'char>>,
    initial_level: u16
}

pub (crate) const EOF_CHAR: char = '\0';

/// True if `c` is considered a whitespace
pub (crate) fn is_whitespace(c: &char) -> bool {
    match c {
        // Usual ASCII suspects
        | '\u{0009}' // \t
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{0020}' // space

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK
            => true,
        _ => false,
    }
}

/// True if `c` is considered an end of line
pub (crate) fn is_end_of_line(c: &char) -> bool {
    match c {
        // Usual ASCII suspects
        | '\u{000A}' // \n
        | '\u{000D}' // \r

        // NEXT LINE from latin1
        | '\u{0085}'

        // // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
            => true,
        _ => false,
    }
}

impl <'c> Cursor<'c> {
    pub fn bump(&mut self) {
        if let Some(ch) = self.ch {
            if is_end_of_line(&ch) {
                self.level();
            }
            else {
                match self.chars.next() {
                    Some(ch) => {
                        self.ch = Some(ch);
                        self.position.1 += 1;
                        self.is_line_start = false;
                    }
                    None => {
                        self.ch = None;
                    }
                }
            }
        }
    }

    fn level(&mut self) {
        assert!(self.ch.is_some() && is_end_of_line(&self.ch.unwrap()));
        self.position.0 += 1;
        self.position.1 = 0;

        match self.first_ahead() {
            EOF_CHAR => {
                self.ch = None;
            },
            mut f_ch if is_whitespace(&f_ch) => {
                let mut new_indent_width = None;
                let mut new_level = self.level + 1;
                let white_space_ch = f_ch.clone();

                while is_whitespace(&f_ch) && white_space_ch == f_ch {
                    self.position.1 += 1;
                    self.ch = self.chars.next();

                    if self.indent_width == 0 {
                        new_indent_width = Some((self.position.1).try_into().unwrap());
                    }
                    else {
                        new_level = self.position.1 / u16::from(self.indent_width) + 1;
                    }

                    f_ch = self.first_ahead();
                }

                self.level = new_level;

                if new_indent_width.is_some() {
                    self.indent_width = new_indent_width.unwrap();
                }

            },
            c if is_end_of_line(&c) => {
                self.ch = self.chars.next();
                self.level();
            },
            _ => {
                self.level = self.initial_level;
                self.ch = self.chars.next();
                self.position.1 += 1;
            }
        }
       
        self.is_line_start = true;
    }

    pub fn first_ahead(&mut self) -> char {
        self.chars.peek().unwrap_or(&EOF_CHAR).clone()
    }
}

impl <'c> From<& 'c str> for Cursor<'c> {
    fn from(input: & 'c str) -> Self {
        let input_c = input.clone(); 
        let mut chars = input_c.chars().peekable();
        Self {
            level: 1,
            ch: chars.next(),
            position: Position(1, 1),
            is_line_start: true,
            indent_width: 0,
            chars,
            initial_level: 1
        }
    }
}

impl <'c> Iterator for Cursor<'c> {
    type Item = Result<RaToken, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ch {
            Some(ch) => match ch {
                ch if is_whitespace(&ch) || is_end_of_line(&ch) => {
                    self.bump();
                    self.next()
                },
                _ => Some(RaToken::try_from(self))
            },
            None => None
        }
    }
}