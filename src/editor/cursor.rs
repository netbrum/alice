use crate::unix::size::TermSize;

use super::{document::Document, position::Position};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Cursor {
    pub position: Position,
    pub offset: Position,
}

impl Cursor {
    pub fn step(&mut self, direction: Direction, document: &Document) {
        let height = document.len();
        let width = document
            .rows
            .get(self.position.y)
            .expect("row at cursor position should exist")
            .len();

        match direction {
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
                if self.position.x < width.saturating_sub(1) {
                    self.position.x = self.position.x.saturating_add(1);
                }
            }
        }

        let row = document.rows.get(self.position.y);

        if let Some(row) = row {
            let length = row.len().saturating_sub(1);

            if self.position.x > length {
                self.position.x = length;
            }
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
}
