csi!(Reset, "H");

pub struct Goto(pub usize, pub usize);

impl std::fmt::Display for Goto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{}H", self.0, self.1)
    }
}
