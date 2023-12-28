use crate::unix::size::TermSize;

use super::document::Document;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Offset {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub offset: Offset,
}

impl Cursor {
    pub fn step(&mut self, direction: Direction, document: &Document) {
        let height = document.len();
        let width = document
            .rows
            .get(self.y)
            .expect("row at cursor position should exist")
            .len();

        match direction {
            Direction::Up => {
                self.y = self.y.saturating_sub(1);
            }
            Direction::Down => {
                if self.y < height.saturating_sub(1) {
                    self.y = self.y.saturating_add(1);
                }
            }
            Direction::Left => {
                self.x = self.x.saturating_sub(1);
            }
            Direction::Right => {
                if self.x < width.saturating_sub(1) {
                    self.x = self.x.saturating_add(1);
                }
            }
        }

        let row = document.rows.get(self.y);

        if let Some(row) = row {
            let length = row.len().saturating_sub(1);

            if self.x > length {
                self.x = length;
            }
        }
    }

    pub fn scroll(&mut self, size: &TermSize) {
        let height = size.height as usize;
        let width = size.width as usize;

        if self.y > self.offset.y.saturating_add(height.saturating_sub(1)) {
            self.offset.y = self.y.saturating_sub(height.saturating_sub(1));
        }

        if self.y < self.offset.y {
            self.offset.y = self.y;
        }

        if self.x > self.offset.x.saturating_add(width.saturating_sub(1)) {
            self.offset.x = self.x.saturating_sub(width.saturating_sub(1));
        }

        if self.x < self.offset.x {
            self.offset.x = self.x;
        }
    }
}
