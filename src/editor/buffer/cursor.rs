mod direction;

use super::Line;

use crate::editor::{Mode, Position};
use crate::escape;
use crate::unix::size::TermSize;

pub use direction::Direction;

use std::{cell::RefCell, fmt::Display, rc::Rc};

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
    }

    pub fn overstep(&mut self, mode: &Mode) {
        let (_, length) = self.size();

        let length = if *mode == Mode::Insert {
            length
        } else {
            length.saturating_sub(1)
        };

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

    pub fn top(&mut self) {
        self.position.y = 0;
    }

    pub fn bottom(&mut self) {
        self.position.y = self.data.borrow().len().saturating_sub(1);
    }

    pub fn start(&mut self) {
        self.position.x = 0;
    }

    pub fn end(&mut self) {
        let (_, length) = self.size();

        self.position.x = length.saturating_sub(1);
    }

    pub fn center(&mut self, size: &TermSize) {
        self.offset.y = self.position.y.saturating_sub(size.height as usize / 2);
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let position = &self.position;
        let offset = &self.offset;

        let y = position.y.saturating_sub(offset.y).saturating_add(1);
        let x = position.x.saturating_sub(offset.x).saturating_add(1);

        write!(f, "{}", escape::cursor::Goto(y, x))
    }
}
