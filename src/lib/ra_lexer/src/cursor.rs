use std::str::Chars;
use std::fmt;
use super::errors::LexerError;

pub const EOF_CHAR: char = '\0';
pub const EOL_CHAR: char = '\n';

#[derive(Copy, Clone, PartialEq)]
pub struct Position(pub u16, pub u16);

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}

pub struct Cursor<'a> {
    input: &'a str,
    initial_len: usize,
    chars: Chars<'a>,
    initial_level: Option<u16>,
    pub position: Position,
    pub level: u16,
    pub indent_width: u16,
}


/// True if `c` is considered a whitespace
pub fn is_whitespace(c: &char) -> bool {
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
pub fn is_end_of_line(c: &char) -> bool {
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

impl <'a> Cursor<'a> {
    pub fn new(input: &'a str, position: Position, level: u16, indent_width: u16) -> Cursor<'a> {
        let initial_level = {
            if level > 0 {
                Some(level)
            }
            else {
                None
            }
        };

        Cursor {
            input,
            initial_len: input.len(),
            chars: input.chars(),
            position,
            level,
            indent_width,
            initial_level
        }
    }

    /// Returns nth character relative to the current cursor position.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    fn nth_char(&self, n: usize) -> char {
        match self.chars().nth(n).unwrap_or(EOF_CHAR) {
            c if is_end_of_line(&c) => EOL_CHAR,
            t => t
        }
    }

    /// Returns a `Chars` iterator over the remaining characters.
    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    /// Peeks the next symbol from the input stream without consuming it.
    pub fn first_ahead(&self) -> char {
        self.nth_char(0)
    }

    /// Peeks the second symbol from the input stream without consuming it.
    pub fn second_ahead(&self) -> char {
        self.nth_char(1)
    }

    /// Checks if there is nothing more to consume.
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().trim().is_empty()
    }

    /// Moves to the next character.
    pub fn bump(&mut self) -> Option<char> {
        let character = self.chars.next();
        match character {
            Some(ch) => {
                if is_end_of_line(&ch) {
                    self.position.0 += 1;
                    self.consume_indent();
                    self.position.1 = self.level * self.indent_width;
                    self.bump()
                }
                else {
                    self.position.1 += 1;
                    Some(ch)
                }
            },
            None => None
        }
    }

    /// Returns amount of already consumed symbols.
    pub fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.input[start..end]
    }

    /// Consumes indent level and returns indentation level
    fn consume_indent(&mut self) {
        if is_whitespace(&self.first_ahead()) {
            if self.indent_width == 0 {
                self.level = {
                    if self.initial_level.is_some() {
                        self.initial_level.unwrap()
                    }
                    else {
                        1
                    }
                };

                self.indent_width = self.eat_while(|c, _| is_whitespace(&c)) / self.level;
            }
            else {
                let initial_level = self.initial_level.clone();
                let indent_width = self.indent_width.clone();

                let inner_width = self.eat_while(|c, eaten| {
                    match initial_level {
                        Some(lvl) => eaten / indent_width  > lvl.to_owned() && is_whitespace(&c),
                        None => is_whitespace(&c)
                    }
                });
                if inner_width % indent_width == 0 {
                    self.level =  inner_width / indent_width;
                } 
                else {
                    panic!(LexerError::UnexpectedIndentLevel(inner_width, self.position));
                }
            }
        }
        else {
            self.level = 0;
        }
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    /// Returns amount of eaten symbols.
    fn eat_while<F>(&mut self, mut predicate: F) -> u16
    where
        F: FnMut(char, &u16) -> bool,
    {
        let mut eaten: u16 = 0;
        while predicate(self.first_ahead(), &eaten) && !self.is_eof() {
            eaten += 1;
            self.bump();
        }

        eaten
    }
}
