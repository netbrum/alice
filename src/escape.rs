use std::fmt::Display;

pub struct CSI<'a>(pub &'a str);

impl<'a> Display for CSI<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}", self.0)
    }
}

pub mod alternate;
pub mod clear;
pub mod cursor;
