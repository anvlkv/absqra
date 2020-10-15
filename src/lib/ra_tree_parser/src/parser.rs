use ra_lexer::errors::LexerError;
use super::*;
use super::errors::TreeParserError;
use super::tree::RaTree;

pub fn parse<'a>(
    mut tokens_stream: impl Iterator<Item = Result<RaToken, LexerError>>
) -> Result<RaTree, (Vec<TreeParserError>, Option<RaTree>)> {
    let mut tree = RaTree::default();

    let mut errors: Vec<TreeParserError> = Vec::new();
    while let Some(token_result) = tokens_stream.next() {
        match token_result {
            Ok(token) => {
                match tree.clone().push_token(token) {
                    Ok(t) => tree = t,
                    Err(e) => errors.extend(e)
                }
            },
            Err(e) => {
                println!("{:?}", e);
                errors.push(TreeParserError::Error);
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