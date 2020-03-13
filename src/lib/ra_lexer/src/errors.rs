#[derive(Debug)]
pub enum LexerError {
    UnexepectedIndentLevel,
    UnexpectedCharacter(char),
    UnexpectedEndOfInput,
    UnexpectedEndOfLine
}