use super::*;

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Result<RaToken, LexerError>> + 'a {
    Cursor::from(input)
    // tokenize_cursor(Cursor::new(input, Position(1, 0), 0, 0)).map(|lexer_result| lexer_result?.try_into())
}