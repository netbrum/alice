use super::CSI;
use std::fmt::Display;

pub const RESET: CSI = CSI("H");
pub const BLINKING_BLOCK: CSI = CSI("1 q");
pub const BLINKING_BAR: CSI = CSI("5 q");
pub const HIDE: CSI = CSI("25l");
pub const SHOW: CSI = CSI("25h");

pub struct Goto(pub usize, pub usize);

impl Display for Goto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{}H", self.0, self.1)
    }
}
