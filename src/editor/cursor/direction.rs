use crate::event::Key;

pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl From<Key> for Direction {
    fn from(key: Key) -> Self {
        match key {
            Key::Char('h') => Self::Left,
            Key::Char('j') => Self::Down,
            Key::Char('k') => Self::Up,
            Key::Char('l') => Self::Right,
            Key::ArrowLeft => Self::Left,
            Key::ArrowDown => Self::Down,
            Key::ArrowUp => Self::Up,
            Key::ArrowRight => Self::Right,
            _ => Self::None,
        }
    }
}
