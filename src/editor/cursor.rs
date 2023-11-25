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
                self.y = self.y.saturating_sub(1);
            }
            Direction::Down => {
                if self.y < (size.height - 1).into() {
                    self.y = self.y.saturating_add(1);
                }
            }
            Direction::Left => {
                self.x = self.x.saturating_sub(1);
            }
            Direction::Right => {
                if self.x < (size.width - 1).into() {
                    self.x = self.x.saturating_add(1);
                }
            }
        }

        // The cursor struct is 0 based, while the ANSI escape codes for the cursor are 1 based, so
        // we transform the values before visually moving the cursor
        let x = self.x.saturating_add(1);
        let y = self.y.saturating_add(1);

        print!("\x1b[{};{}H", y, x);
    }
}
