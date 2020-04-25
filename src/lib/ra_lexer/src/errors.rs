#[derive(Debug)]
pub enum LexerError {
    UnexpectedIndentLevel,
    UnexpectedCharacter(char),
    UnexpectedEndOfInput,
    UnexpectedEndOfLine,
    InvalidNumber
}