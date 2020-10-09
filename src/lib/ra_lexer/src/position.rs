use std::fmt;
use serde::{Serialize};

#[derive(Copy, Clone, PartialEq, Serialize)]
pub struct Position(pub u16, pub u16);

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}