use super::Size;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn step(&mut self, direction: Direction, size: &Size) {
        match direction {
            Direction::Up => {
                self.y = self.y.saturating_sub(1);
            }
            Direction::Down => {
                if self.y < (size.height - 1) {
                    self.y = self.y.saturating_add(1);
                }
            }
            Direction::Left => {
                self.x = self.x.saturating_sub(1);
            }
            Direction::Right => {
                if self.x < (size.width - 1) {
                    self.x = self.x.saturating_add(1);
                }
            }
        }
    }
}
