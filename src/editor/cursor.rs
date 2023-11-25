use super::Size;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn step(&mut self, direction: Direction, size: &Size) {
        match direction {
            Direction::Up => {
                if self.y > 1 {
                    self.y = self.y.saturating_sub(1);
                }
            }
            Direction::Down => {
                if self.y < size.height.into() {
                    self.y = self.y.saturating_add(1);
                }
            }
            Direction::Left => {
                if self.x > 1 {
                    self.x = self.x.saturating_sub(1);
                }
            }
            Direction::Right => {
                if self.x < size.width.into() {
                    self.x = self.x.saturating_add(1);
                }
            }
        }

        print!("\x1b[{};{}H", self.y, self.x);
    }
}
