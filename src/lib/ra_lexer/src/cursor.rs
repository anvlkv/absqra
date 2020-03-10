use std::str::Chars;
use std::fmt;
pub(crate) const EOF_CHAR: char = '\0';
pub(crate) const EOL_CHAR: char = '\n';

pub(crate) struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
    pub position: Position,
    pub level: usize
}

#[derive(Copy, Clone, PartialEq)]
pub struct Position(pub usize, pub usize);

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}



/// True if `c` is considered a whitespace
pub fn is_whitespace(c: char) -> bool {
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
pub fn is_end_of_line(c: char) -> bool {
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
    pub(crate)fn new(input: &'a str, position: Position, level: usize) -> Cursor<'a> {
        Cursor {
            initial_len: input.len(),
            chars: input.chars(),
            position,
            level
        }
    }

    /// Returns nth character relative to the current cursor position.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    fn nth_char(&self, n: usize) -> char {
        match self.chars().nth(n).unwrap_or(EOF_CHAR) {
            c if is_end_of_line(c) => EOL_CHAR,
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
        self.chars.as_str().is_empty()
    }

    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        let character = self.chars.next();
        match character {
            Some(ch) => {
                match ch {
                    c if is_end_of_line(c) => {
                        self.position.0 += 1;
                        self.consume_indent();
                        self.position.1 = self.level;
                        self.bump()
                    }
                    c => {
                        self.position.1 += 1;
                        Some(c)
                    }
                }
            },
            None => None
        }
        
    }

    /// Returns amount of already consumed symbols.
    pub(crate) fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    /// Consumes indent level and returns indentation level
    fn consume_indent(&mut self) {
        self.level = self.eat_while(|c| is_whitespace(c));
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    /// Returns amount of eaten symbols.
    fn eat_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut eaten: usize = 0;
        while predicate(self.first_ahead()) && !self.is_eof() {
            eaten += 1;
            self.bump();
        }

        eaten
    }
}

#[cfg(test)]
mod tests {
    use super::{Cursor, EOF_CHAR, Position};
    #[test]
    fn it_should_create() {
        Cursor::new("abc", Position(1, 0), 0);
    }

    #[test]
    fn it_should_give_first_char() {
        let cur = Cursor::new("abc", Position(1, 0), 0);
        assert_eq!(cur.first_ahead(), 'a');
    }

    #[test]
    fn it_should_give_second_char() {
        let cur = Cursor::new("abc", Position(1, 0), 0);
        assert_eq!(cur.second_ahead(), 'b');
    }

    #[test]
    fn it_should_return_next_char() {
        let mut cur = Cursor::new("a", Position(1, 0), 0);
        assert_eq!(cur.bump().unwrap(), 'a');
    }

    #[test]
    fn it_should_return_none_at_end_of_input() {
        let mut cur = Cursor::new("a", Position(1, 0), 0);
        cur.bump();
        assert_eq!(cur.bump(), None);
    }

    #[test]
    fn it_should_confirm_eof() {
        let mut cur = Cursor::new("a", Position(1, 0), 0);
        cur.bump();
        cur.bump();
        cur.bump();
        assert_eq!(cur.is_eof(), true);
    }

    #[test]
    fn it_should_return_amount_of_consumed_symbols() {
        let mut cur = Cursor::new("abc", Position(1, 0), 0);
        cur.bump();
        cur.bump();
        cur.bump();
        assert_eq!(cur.len_consumed(), 3);
    }

    #[test]
    fn it_should_not_panic_when_encoutering_funny_characters() {
        let mut cur = Cursor::new("🚬", Position(1, 0), 0);
        assert_eq!(cur.bump().unwrap(), '🚬');
    }

    #[test]
    fn it_should_track_position() {
        let mut cur = Cursor::new("abc\nbca", Position(1, 0), 0);
        assert_eq!(cur.bump().unwrap(), 'a');
        assert_eq!(cur.position, Position(1, 1));
        assert_eq!(cur.bump().unwrap(), 'b');
        assert_eq!(cur.position, Position(1, 2));
        assert_eq!(cur.bump().unwrap(), 'c');
        assert_eq!(cur.position, Position(1, 3));
        assert_eq!(cur.bump().unwrap(), 'b');
        assert_eq!(cur.position, Position(2, 1));
        assert_eq!(cur.bump().unwrap(), 'c');
        assert_eq!(cur.position, Position(2, 2));
        assert_eq!(cur.bump().unwrap(), 'a');
        assert_eq!(cur.position, Position(2, 3));
    }

    #[test]
    fn it_should_track_indent_level() {
        let mut cur = Cursor::new("a\n\tb\n\t\tc\nd", Position(1, 0), 0);
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