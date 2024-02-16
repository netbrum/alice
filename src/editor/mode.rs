use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    Exit,
    Normal,
    Insert,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Self::Exit => "Exit",
            Self::Normal => "Normal",
            Self::Insert => "Insert",
            Self::Command => "Command",
        };

        write!(f, "{mode}")
    }
}
