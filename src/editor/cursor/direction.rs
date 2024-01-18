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
            Key::Char('h') => Direction::Left,
            Key::Char('j') => Direction::Down,
            Key::Char('k') => Direction::Up,
            Key::Char('l') => Direction::Right,
            Key::ArrowLeft => Direction::Left,
            Key::ArrowDown => Direction::Down,
            Key::ArrowUp => Direction::Up,
            Key::ArrowRight => Direction::Right,
            _ => Direction::None,
        }
    }
}
