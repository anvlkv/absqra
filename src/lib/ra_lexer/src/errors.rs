#[derive(Debug)]
pub enum LexerError {
    UnexpectedIndentLevel,
    UnexpectedCharacter(char),
    UnsupportedToken,
    UnexpectedEndOfInput,
    UnexpectedEndOfLine,
    InvalidNumber
}