mod row;

pub use row::Row;

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
}
