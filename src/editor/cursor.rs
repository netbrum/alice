mod direction;

pub use direction::Direction;

use super::{Buffer, Position};

use crate::unix::size::TermSize;

#[derive(Default)]
pub struct Cursor {
    pub position: Position,
    pub offset: Position,
}

impl Cursor {
    pub fn step(&mut self, direction: Direction, buffer: &Buffer) {
        let height = buffer.len();

        match direction {
            Direction::None => {}
            Direction::Up => {
                self.position.y = self.position.y.saturating_sub(1);
            }
            Direction::Down => {
                if self.position.y < height.saturating_sub(1) {
                    self.position.y = self.position.y.saturating_add(1);
                }
            }
            Direction::Left => {
                self.position.x = self.position.x.saturating_sub(1);
            }
            Direction::Right => {
                let line = buffer
                    .lines
                    .get(self.position.y)
                    .expect("line at cursor position should exist");

                let length = line.len().saturating_sub(1);

                if self.position.x < length {
                    self.position.x = self.position.x.saturating_add(1);
                }
            }
        }

        self.overstep(buffer);
    }

    pub fn overstep(&mut self, buffer: &Buffer) {
        let line = buffer
            .lines
            .get(self.position.y)
            .expect("line at cursor position should exist");

        let length = line.len().saturating_sub(1);

        if self.position.x > length {
            self.position.x = length;
        }
    }

    pub fn scroll(&mut self, size: &TermSize) {
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        if self.position.y > self.offset.y.saturating_add(height) {
            self.offset.y = self.position.y.saturating_sub(height);
        }

        if self.position.y < self.offset.y {
            self.offset.y = self.position.y;
        }

        if self.position.x > self.offset.x.saturating_add(width) {
            self.offset.x = self.position.x.saturating_sub(width);
        }

        if self.position.x < self.offset.x {
            self.offset.x = self.position.x;
        }
    }

    pub fn backspace(&mut self, buffer: &Buffer) {
        if self.position.x > 0 {
            self.position.x -= 1;
            return;
        }

        if self.position.y > 0 {
            self.position.y -= 1;

            let line = buffer.lines.get(self.position.y);
            self.position.x = if let Some(line) = line { line.len() } else { 0 };
        }
    }
}
