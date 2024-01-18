mod direction;

pub use direction::Direction;

use super::{document::Document, position::Position};

use crate::unix::size::TermSize;

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
                if self.position.x < width.saturating_sub(1) {
                    self.position.x = self.position.x.saturating_add(1);
                }
            }
        }

        self.overstep(document);
    }

    pub fn overstep(&mut self, document: &Document) {
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

    pub fn backspace(&mut self, document: &Document) {
        if self.position.x > 0 {
            self.position.x -= 1;
            return;
        }

        if self.position.y > 0 {
            self.position.y -= 1;

            self.position.x = if let Some(row) = document.rows.get(self.position.y) {
                row.len()
            } else {
                0
            }
        }
    }
}
