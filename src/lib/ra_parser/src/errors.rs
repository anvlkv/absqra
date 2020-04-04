use ra_lexer::Token;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedIndentLevel,
    UnexpectedToken(Token),
    ExpectedAGotB(Token, Token),
    UnexpectedEndOfInput
}