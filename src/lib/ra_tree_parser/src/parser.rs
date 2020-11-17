use ra_lexer::errors::LexerError;
use super::*;
use super::tree::RaTree;

pub fn parse<'a>(
    mut tokens_stream: impl Iterator<Item = Result<RaToken, LexerError>>
) -> Result<RaTree, (Vec<TreeParserError>, Option<RaTree>)> {
    let mut tree = RaTree::default();

    let mut errors: Vec<TreeParserError> = Vec::new();
    while let Some(token_result) = tokens_stream.next() {
        match token_result {
            Ok(token) => {
                match tree.push_token(token) {
                    Err(e) => errors.extend(e),
                    _ => {}
                }
            },
            Err(e) => {
                errors.push(TreeParserError::LexerError(e, Backtrace::new()));
            }
        }
    }
    if errors.len() > 0 {
        Err((errors, Some(tree)))
    }
    else {
        Ok(tree)
    }
}