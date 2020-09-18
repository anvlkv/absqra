use crate::errors::ParserError;

pub trait Finalizable<T>
where T: Sized {
    fn is_final(&self) -> bool;
    fn is_ready_to_finalize(&self) -> bool;
    fn finalize(self) -> Result<Box<T>, ParserError>;
}