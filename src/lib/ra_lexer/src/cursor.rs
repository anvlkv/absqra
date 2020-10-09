use super::*;
// use std::str::Chars;
use std::convert::{TryInto, TryFrom};

pub (crate) struct Cursor {
    pub position: Position,
    pub level: u16,
    pub ch: Option<char>,
    pub is_line_start: bool,
    input: String,
    idx: usize,
    indent_width: u8
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

impl Cursor {
    pub fn bump(&mut self) {
        let idx = self.idx + 1;
        match self.input.chars().nth(idx) {
            Some(ch) => {
                self.idx = idx;
                self.ch = Some(ch);
                if is_end_of_line(&ch) {
                    self.position.0 += 1;
                    self.position.1 = 0;
                    self.level();
                }
                else {
                    self.position.1 += 1;
                    self.is_line_start = false;
                }
            }
            None => {
                self.ch = None;
            }
        }
    }

    fn level(&mut self) {
        match self.first_ahead() {
            EOF_CHAR => {
                self.ch = None;
            },
            mut ch => {
                let mut new_indent_width = None;
                let mut new_level = None;
                if is_whitespace(&ch) {
                    let white_space_ch = ch.clone();
                    while is_whitespace(&ch) && white_space_ch == ch {
                        if self.indent_width == 0 {
                            new_level = Some(self.level + 1);
                            new_indent_width = Some((self.position.1).try_into().unwrap());
                        }
                        else {
                            new_level = Some(self.position.1 / u16::from(self.indent_width));
                        }

                        self.bump();
                        ch = self.first_ahead();
                    }
                }
                else {
                    self.bump();
                }
                self.level = new_level.unwrap_or(self.level);
                if new_indent_width.is_some() {
                    self.indent_width = new_indent_width.unwrap();
                }
                // self.ch = Some(ch);
                // self.idx += 1;
            }
        }
       
        self.is_line_start = true;
    }

    pub fn first_ahead(&self) -> char {
        self.input.chars().nth(self.idx + 1).unwrap_or(EOF_CHAR)
    }
}

impl <'a> From<& 'a str> for Cursor {
    fn from(input: & 'a str) -> Self { 
        let ch = input.chars().nth(0);
        Self {
            input: input.to_owned(),
            level: 1,
            ch,
            position: Position(1, 1),
            is_line_start: true,
            idx: 0,
            indent_width: 0
        }
    }
}

impl Iterator for Cursor {
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