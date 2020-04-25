use std::str::Chars;
use std::fmt;
use super::errors::LexerError;
use std::convert::TryInto;

pub(crate) const EOF_CHAR: char = '\0';
pub(crate) const EOL_CHAR: char = '\n';

#[derive(Copy, Clone, PartialEq)]
pub struct Position(pub u16, pub u16);

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}

pub(crate) struct Cursor<'a> {
    input: &'a str,
    initial_len: usize,
    chars: Chars<'a>,
    is_reading_continuous_block_at: (bool, u16),
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
    pub(crate)fn new(input: &'a str, position: Position, level: u16, indent_width: u16) -> Cursor<'a> {
        Cursor {
            input,
            initial_len: input.len(),
            chars: input.chars(),
            position,
            level,
            indent_width,
            is_reading_continuous_block_at: (false, 0)
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
    pub(crate) fn first_ahead(&self) -> char {
        self.nth_char(0)
    }

    /// Peeks the second symbol from the input stream without consuming it.
    pub(crate) fn second_ahead(&self) -> char {
        self.nth_char(1)
    }

    /// Checks if there is nothing more to consume.
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().trim().is_empty()
    }

    pub(crate) fn start_reading_continuous_block(&mut self) {
        if !self.is_reading_continuous_block_at.0 {
            self.is_reading_continuous_block_at = (true, self.level);
        }
    }

    pub(crate) fn end_reading_continuous_block(&mut self) {
        self.is_reading_continuous_block_at = (false, 0)
    }

    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        let character = self.chars.next();
        match character {
            Some(ch) => {
                if is_end_of_line(&ch) {
                    self.position.0 += 1;
                    self.consume_indent(match self.is_reading_continuous_block_at.0 {
                        true => Some(self.is_reading_continuous_block_at.1),
                        false => None
                    });
                    self.position.1 = self.level * self.indent_width;
                    if self.is_reading_continuous_block_at.0 {
                        Some(ch)
                    }
                    else {
                        self.bump()
                    }
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
    pub(crate) fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub(crate) fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.input[start..end]
    }

    /// Consumes indent level and returns indentation level
    fn consume_indent(&mut self, limit: Option<u16>) {
        if self.indent_width == 0 {
            self.level = 1;
            self.indent_width = self.eat_while(|c, eaten| is_whitespace(&c) && match limit {
                Some(l) => eaten <= &l,
                None => true
            });
        }
        else {
            let indent_width = self.indent_width;
            let inner_width = self.eat_while(|c, eaten| is_whitespace(&c) && match limit {
                Some(l) => eaten < &(l * indent_width),
                None => true
            });
            if inner_width % self.indent_width == 0 {
                self.level =  inner_width / self.indent_width;
            } 
            else {
                panic!(LexerError::UnexpectedIndentLevel);
            }
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

#[cfg(test)]
mod tests {
    use super::{Cursor, Position};
    #[test]
    fn it_should_create() {
        Cursor::new("abc", Position(1, 0), 0, 0);
    }

    #[test]
    fn it_should_give_first_char() {
        let cur = Cursor::new("abc", Position(1, 0), 0, 0);
        assert_eq!(cur.first_ahead(), 'a');
    }

    #[test]
    fn it_should_give_second_char() {
        let cur = Cursor::new("abc", Position(1, 0), 0, 0);
        assert_eq!(cur.second_ahead(), 'b');
    }

    #[test]
    fn it_should_return_next_char() {
        let mut cur = Cursor::new("a", Position(1, 0), 0, 0);
        assert_eq!(cur.bump().unwrap(), 'a');
    }

    #[test]
    fn it_should_return_none_at_end_of_input() {
        let mut cur = Cursor::new("a", Position(1, 0), 0, 0);
        cur.bump();
        assert_eq!(cur.bump(), None);
    }

    #[test]
    fn it_should_confirm_eof() {
        let mut cur = Cursor::new("a", Position(1, 0), 0, 0);
        cur.bump();
        cur.bump();
        cur.bump();
        assert_eq!(cur.is_eof(), true);
    }

    #[test]
    fn it_should_return_amount_of_consumed_symbols() {
        let mut cur = Cursor::new("abc", Position(1, 0), 0, 0);
        cur.bump();
        cur.bump();
        cur.bump();
        assert_eq!(cur.len_consumed(), 3);
    }

    #[test]
    fn it_should_not_panic_when_encountering_funny_characters() {
        let mut cur = Cursor::new("🚬", Position(1, 0), 0, 0);
        assert_eq!(cur.bump().unwrap(), '🚬');
    }

    #[test]
    fn it_should_track_position() {
        let mut cur = Cursor::new("abc\nSOME", Position(1, 0), 0, 0);
        assert_eq!(cur.bump().unwrap(), 'a');
        assert_eq!(cur.position, Position(1, 1));
        assert_eq!(cur.bump().unwrap(), 'b');
        assert_eq!(cur.position, Position(1, 2));
        assert_eq!(cur.bump().unwrap(), 'c');
        assert_eq!(cur.position, Position(1, 3));
        assert_eq!(cur.bump().unwrap(), 'S');
        assert_eq!(cur.position, Position(2, 1));
        assert_eq!(cur.bump().unwrap(), 'O');
        assert_eq!(cur.position, Position(2, 2));
        assert_eq!(cur.bump().unwrap(), 'M');
        assert_eq!(cur.position, Position(2, 3));
    }

    #[test]
    fn it_should_track_indent_level() {
        let mut cur = Cursor::new("a\n\tb\n\t\tc\nd", Position(1, 0), 0, 0);
        assert_eq!(cur.bump().unwrap(), 'a');
        assert_eq!(cur.level, 0);
        assert_eq!(cur.bump().unwrap(), 'b');
        assert_eq!(cur.level, 1);
        assert_eq!(cur.bump().unwrap(), 'c');
        assert_eq!(cur.level, 2);
        assert_eq!(cur.position, Position(3, 3));
        assert_eq!(cur.bump().unwrap(), 'd');
        assert_eq!(cur.level, 0);
    }
}