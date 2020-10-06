extern crate failure;
use failure::{Fail, Backtrace};


#[derive(Debug, Fail)]
pub enum TreeParserError {
    #[fail(display = "Invalid tree")]
    Error
}