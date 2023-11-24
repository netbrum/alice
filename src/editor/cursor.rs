use super::Size;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    pub fn new(x: u16, y: u16) -> Self {
        Cursor { x, y }
    }

    pub fn step(&mut self, direction: Direction, size: &Size) {
        match direction {
            Direction::Up => {
                if self.y > 1 {
                    self.y = self.y.saturating_sub(1);
                }
            }
            Direction::Down => {
                if self.y < size.height {
                    self.y = self.y.saturating_add(1);
                }
            }
            Direction::Left => {
                if self.x > 1 {
                    self.x = self.x.saturating_sub(1);
                }
            }
            Direction::Right => {
                if self.x < size.width {
                    self.x = self.x.saturating_add(1);
                }
            }
        }

        print!("\x1b[{};{}H", self.y, self.x);
    }
}
