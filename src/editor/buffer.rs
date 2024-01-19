mod row;

pub use row::Row;

use super::position::Position;

use std::{
    fs::File,
    io::{Read, Result},
    path::PathBuf,
};

pub struct Buffer {
    pub rows: Vec<Row>,
    path: Option<PathBuf>,
}

impl Buffer {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let rows = data.lines().map(Row::from).collect();

        Ok(Buffer {
            rows,
            path: Some(path.canonicalize()?),
        })
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn newline(&mut self, position: &Position) {
        let row = self
            .rows
            .get_mut(position.y)
            .expect("row at cursor position should exist");

        let new = row.split(position.x);

        self.rows.insert(position.y.saturating_add(1), new);
    }

    pub fn insert(&mut self, position: &Position, character: char) {
        if character == '\n' {
            self.newline(position);
        } else {
            let row = self
                .rows
                .get_mut(position.y)
                .expect("row at cursor position should exist");

            row.insert(position.x, character);
        }
    }

    pub fn delete(&mut self, position: &Position) {
        let length = self.len();

        let row = self
            .rows
            .get_mut(position.y)
            .expect("row at cursor position should exist");

        // Since the position struct is zero based, we know to combine the rows whenever the x
        // position is the same as the row length, which is one to the right of the row
        if position.x == row.len() && position.y.saturating_add(1) < length {
            let next = self.rows.remove(position.y.saturating_add(1));
            self.rows[position.y].append(&next.to_string());
        } else {
            row.delete(position.x);
        }
    }
}
