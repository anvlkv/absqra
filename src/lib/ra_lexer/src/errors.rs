#[derive(Debug)]
pub enum LexerError {
    UnexepectedIndentLevel,
    UnexpectedCharacter(char)
}