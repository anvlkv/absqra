use super::{ParsedByToken, ParserError};
use std::rc::Rc;
use ra_lexer::token::RaToken;
use failure::Backtrace;


pub(crate) trait Buffered<'a, T> : ParsedByToken<'a, T>
where T: ParsedByToken<'a, T> {
    
    fn new_candidates_from_token(token: &RaToken<'a>) -> Vec<T>;
    fn get_buffer(&self) -> Vec<Rc<T>>;
    fn get_candidates_for_token(&self, token: &RaToken<'a>) -> Result<Vec<T>, Vec<ParserError>> {
        let candidates = self.get_buffer().clone();
        let mut result = Vec::new();
        let mut errors = Vec::new();
        
        if candidates.len() == 0 {
            result = Self::new_candidates_from_token(token);
        }
        else {
            let mut candidates_iter = candidates.iter();

            while let Some(candidate) = candidates_iter.next() {
                if candidate.allowed_tokens().contains(&token.kind) {
                    match candidate.as_ref().clone().append_token(*token) {
                        Ok(c) => result.push(*c),
                        Err(e) => errors.extend(e)
                    }
                }
            }
        }

        if result.len() == 0 {
            errors.push(ParserError::InvalidExpression(token.position.0, Backtrace::new()));
            Err(errors)
        }
        else {
            Ok(result)
        }
    }


    // fn get_start_candidates(token: RaToken<'a>, src: Vec<&impl ParsedByToken<'a>>) -> Vec<dyn ParsedByToken<'a>>;
}
