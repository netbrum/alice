mod line;

pub use line::Line;

use super::position::Position;

use std::{
    fs::File,
    io::{Read, Result},
    path::PathBuf,
};

pub struct Buffer {
    pub lines: Vec<Line>,
    path: Option<PathBuf>,
}

impl Buffer {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let lines = data.lines().map(Line::from).collect();

        Ok(Self {
            lines,
            path: Some(path.canonicalize()?),
        })
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn newline(&mut self, position: &Position) {
        let line = self
            .lines
            .get_mut(position.y)
            .expect("line at cursor position should exist");

        let new = line.split(position.x);

        self.lines.insert(position.y.saturating_add(1), new);
    }

    pub fn insert(&mut self, position: &Position, character: char) {
        if character == '\n' {
            self.newline(position);
        } else {
            let line = self
                .lines
                .get_mut(position.y)
                .expect("line at cursor position should exist");

            line.insert(position.x, character);
        }
    }

    pub fn delete(&mut self, position: &Position) {
        let length = self.len();

        let line = self
            .lines
            .get_mut(position.y)
            .expect("line at cursor position should exist");

        // Since the position struct is zero based, we know to combine the lines whenever the x
        // position is the same as the line length, which is one to the right of the line
        if position.x == line.len() && position.y.saturating_add(1) < length {
            let next = self.lines.remove(position.y.saturating_add(1));
            self.lines[position.y].append(&next.to_string());
        } else {
            line.delete(position.x);
        }
    }
}
