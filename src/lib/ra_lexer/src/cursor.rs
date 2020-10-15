use super::*;
use core::iter::Peekable;
use std::str::Chars;
use std::convert::{TryInto, TryFrom};

pub (crate) struct Cursor<'char> {
    pub position: Position,
    pub level: u16,
    pub ch: Option<char>,
    pub is_line_start: bool,
    pub ruler: Option<u16>,
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
            match ch {
                c if is_end_of_line(&c) => {
                    self.level();
                },
                _ => {
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
                let mut new_level = 0;
                let white_space_ch = f_ch.clone();
                let limit = match self.ruler {
                    Some(l) => l,
                    None => std::u16::MAX
                };
                
                while is_whitespace(&f_ch) && white_space_ch == f_ch {
                    self.position.1 += 1;
                    self.ch = self.chars.next();
                    
                    
                    if self.indent_width == 0 {
                        new_indent_width = Some((self.position.1).try_into().unwrap());
                    }
                    else {
                        let indent_16 = u16::from(self.indent_width);
                        if self.position.1 % indent_16  == 0 {
                            new_level = self.position.1 / indent_16;
                        }
                    }

                    f_ch = self.first_ahead();

                    if new_level >= limit {
                        break;
                    }
                }

                self.position.1 += 1;
                self.ch = self.chars.next();

                match new_indent_width {
                    Some(width) => {
                        self.indent_width = width;
                        self.level = 2;
                    },
                    None => {
                        self.level = new_level + 1;
                    }
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
            ruler: None,
            indent_width: 0,
            chars,
            initial_level: 1,
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