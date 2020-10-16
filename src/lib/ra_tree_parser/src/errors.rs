use super::*;


#[derive(Debug, Fail)]
pub enum TreeParserError {
    #[fail(display = "Encountered an error: {} {}", _0, _1)]
    LexerError(#[cause] ra_lexer::errors::LexerError, Backtrace),
    #[fail(display = "Built invalid tree: {} {}", _0, _1)]
    InvalidTree(Position, Backtrace)
}