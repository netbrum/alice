use std::fmt::Display;

#[derive(Debug)]
pub enum Key {
    Escape,
    Insert,
    Delete,
    Home,
    End,
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    PageUp,
    PageDown,
    Tab,
    Enter,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Null,
    Alt(char),
    Ctrl(char),
    Char(char),
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match self {
            Self::Escape => String::from("Escape"),
            Self::Insert => String::from("Insert"),
            Self::Delete => String::from("Delete"),
            Self::Home => String::from("Home"),
            Self::End => String::from("End"),
            Self::Backspace => String::from("Backspace"),
            Self::ArrowUp => String::from("ArrowUp"),
            Self::ArrowDown => String::from("ArrowDown"),
            Self::ArrowLeft => String::from("ArrowLeft"),
            Self::ArrowRight => String::from("ArrowRight"),
            Self::PageUp => String::from("PageUp"),
            Self::PageDown => String::from("PageDown"),
            Self::Tab => String::from("Tab"),
            Self::Enter => String::from("Enter"),
            Self::F1 => String::from("F1"),
            Self::F2 => String::from("F2"),
            Self::F3 => String::from("F3"),
            Self::F4 => String::from("F4"),
            Self::F5 => String::from("F5"),
            Self::F6 => String::from("F6"),
            Self::F7 => String::from("F7"),
            Self::F8 => String::from("F8"),
            Self::F9 => String::from("F9"),
            Self::F10 => String::from("F10"),
            Self::F11 => String::from("F11"),
            Self::F12 => String::from("F12"),
            Self::Null => String::from("Null"),
            Self::Alt(character) => character.to_string(),
            Self::Ctrl(character) => character.to_string(),
            Self::Char(character) => character.to_string(),
        };

        write!(f, "{key}")
    }
}
