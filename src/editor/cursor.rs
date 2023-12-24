use super::document::Document;

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
}
