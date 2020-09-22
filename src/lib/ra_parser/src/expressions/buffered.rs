use super::*;

pub type Buffer<T> = Vec<Rc<T>>;

pub(crate) trait Buffered<'a, T>: ParsedByToken<'a, T>
where
    T: ParsedByToken<'a, T>,
{
    fn new_candidates_from_token(token: &RaToken<'a>) -> Buffer<T>;
    fn get_buffer(&self) -> Buffer<T>;
    fn get_candidates_for_token(&self, token: &RaToken<'a>) -> Result<Buffer<T>, Vec<ParserError>> {
        let candidates = self.get_buffer().clone();
        let mut result = Vec::new();
        let mut errors = Vec::new();
        if candidates.len() == 0 {
            result = Self::new_candidates_from_token(token);
        } else {
            let mut candidates_iter = candidates.iter();

            while let Some(candidate) = candidates_iter.next() {
                if candidate.allowed_tokens().contains(&token.kind) {
                    match candidate.as_ref().clone().append_token(*token) {
                        Ok(c) => result.push(Rc::new(*c)),
                        Err(e) => errors.extend(e),
                    }
                }
            }
        }

        if result.len() == 0 {
            errors.push(ParserError::InvalidExpression(
                token.position.0,
                Backtrace::new(),
            ));
            Err(errors)
        } else {
            Ok(result)
        }
    }
}
