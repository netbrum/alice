mod direction;

use super::Line;

use crate::editor::Position;
use crate::unix::size::TermSize;

pub use direction::Direction;

use std::{cell::RefCell, rc::Rc};

pub struct Cursor {
    pub position: Position,
    pub offset: Position,
    data: Rc<RefCell<Vec<Line>>>,
}

impl Cursor {
    pub fn new(data: Rc<RefCell<Vec<Line>>>) -> Self {
        Self {
            position: Position::default(),
            offset: Position::default(),
            data,
        }
    }

    fn size(&self) -> (usize, usize) {
        let data = self.data.borrow();

        let height = data.len();
        let length = if let Some(line) = data.get(self.position.y) {
            line.len()
        } else {
            0
        };

        (height, length)
    }

    pub fn step(&mut self, direction: Direction) {
        let (height, length) = self.size();

        match direction {
            Direction::None => (),
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
                if self.position.x < length {
                    self.position.x = self.position.x.saturating_add(1);
                }
            }
        }

        self.overstep();
    }

    pub fn overstep(&mut self) {
        let (_, length) = self.size();

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

    pub fn backspace(&mut self) {
        if self.position.x > 0 {
            self.position.x -= 1;
        } else if self.position.y > 0 {
            self.position.y -= 1;

            let (_, length) = self.size();
            self.position.x = length;
        }
    }
}
