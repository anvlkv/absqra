use super::*;

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Result<RaToken, LexerError>> + 'a {
    Cursor::from(input)
}