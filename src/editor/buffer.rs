pub mod cursor;
pub mod line;

use cursor::Cursor;
use line::Line;

use std::{
    cell::{Ref, RefCell},
    ffi::OsStr,
    fs::File,
    io::{Read, Result, Write},
    path::PathBuf,
    rc::Rc,
};

pub struct Buffer {
    pub cursor: Cursor,
    data: Rc<RefCell<Vec<Line>>>,
    path: PathBuf,
}

impl Buffer {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut data = String::new();

        file.read_to_string(&mut data)?;

        let mut lines: Vec<Line> = data.lines().map(Line::from).collect();

        if lines.is_empty() {
            lines.push(Line::default());
        }

        let data = Rc::new(RefCell::new(lines));
        let cursor = Cursor::new(Rc::clone(&data));
        let path = path.canonicalize()?;

        Ok(Self { data, cursor, path })
    }

    pub fn data(&self) -> Ref<'_, Vec<Line>> {
        self.data.borrow()
    }

    pub fn file_name(&self) -> String {
        let file_name = self
            .path
            .file_name()
            .unwrap_or(OsStr::new("Unknown"))
            .to_str()
            .unwrap_or("Unknown");

        String::from(file_name)
    }

    pub fn newline(&self) {
        let mut lines = self.data.borrow_mut();

        if let Some(line) = lines.get_mut(self.cursor.position.y) {
            let new = line.split(self.cursor.position.x);

            lines.insert(self.cursor.position.y.saturating_add(1), new)
        }
    }

    pub fn insert(&mut self, character: char) {
        let mut lines = self.data.borrow_mut();

        if character == '\n' {
            self.newline();
        } else if let Some(line) = lines.get_mut(self.cursor.position.y) {
            line.insert(self.cursor.position.x, character);
        }
    }

    pub fn delete(&mut self) {
        let mut lines = self.data.borrow_mut();

        let position = self.cursor.position;
        let length = lines.len();

        if let Some(line) = lines.get_mut(position.y) {
            // Since the position struct is zero based, we know to combine the lines whenever the x
            // position is the same as the line length, which is one to the right of the line
            if position.x == line.len() && position.y.saturating_add(1) < length {
                let next = lines.remove(position.y.saturating_add(1));
                lines[position.y].append(&next.to_string());
            } else {
                line.delete(position.x);
            }
        }
    }

    pub fn save(&self) -> Result<usize> {
        let mut file = File::create(&self.path)?;

        let lines = self.data();

        // This is probably not very effective, as to_vec copies the byte slice, however,
        // using an iterator to collect the bytes and writing to file once is preferable
        //
        // Doing it with an iterator without copying the slice to append the newline is probably
        // not possible due to rust's ownership rules
        let data: Vec<u8> = lines
            .iter()
            .flat_map(|line| {
                let mut bytes = line.as_bytes().to_vec();
                bytes.push(b'\n');

                bytes
            })
            .collect();

        file.write(&data)
    }
}
