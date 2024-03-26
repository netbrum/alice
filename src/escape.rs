use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct CSI(pub &'static str);

impl Display for CSI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}", self.0)
    }
}

pub mod alternate;
pub mod clear;
pub mod color;
pub mod cursor;
