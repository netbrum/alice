mod row;

pub use row::Row;

use super::position::Position;

use std::{fs, io::Result, path::PathBuf};

pub struct Document {
    pub rows: Vec<Row>,
    path: PathBuf,
}

impl Document {
    pub fn open(path: &PathBuf) -> Result<Self> {
        let buffer = fs::read(path);

        match buffer {
            Err(error) => {
                if path.exists() {
                    Err(error)
                } else {
                    Ok(Document {
                        rows: vec![Row::default()],
                        path: path.to_path_buf(),
                    })
                }
            }
            Ok(contents) => {
                let rows = String::from_utf8_lossy(&contents)
                    .lines()
                    .map(Row::from)
                    .collect();

                Ok(Document {
                    rows,
                    path: path.to_path_buf(),
                })
            }
        }
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
