use serde::Serialize;
use std::fmt;
use std::ops::{Add, AddAssign};

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

impl Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        let res = self.clone() + other;
        self.0 = res.0;
        self.1 = res.1;
    }
}
