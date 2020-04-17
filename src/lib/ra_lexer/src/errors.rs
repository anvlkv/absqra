#[derive(Debug)]
pub enum LexerError<'a> {
    UnexpectedIndentLevel,
    UnexpectedCharacter(&'a char),
    UnexpectedEndOfInput,
    UnexpectedEndOfLine
}