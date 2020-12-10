use super::*;
use ra_lexer::errors::LexerError;

pub fn parse<'a>(
    mut tokens_stream: impl Iterator<Item = Result<RaToken, LexerError>>
) -> Result<RaTree, (Vec<LexerError>, Option<RaTree>)> {

    let mut tree = RaTree::new();

    let mut errors: Vec<LexerError> = Vec::new();

    while let Some(token_result) = tokens_stream.next() {
        match token_result {
            Ok(token) => {
                tree.add_token(token);
            },
            Err(e) => {
                errors.push(e);
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
