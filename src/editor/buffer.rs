pub mod line;

use super::Position;

use line::Line;

use std::{
    cell::{Ref, RefCell},
    fs::File,
    io::{Read, Result},
    path::PathBuf,
    rc::Rc,
};

pub struct Buffer {
    data: Rc<RefCell<Vec<Line>>>,
    path: Option<PathBuf>,
}

impl Buffer {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut data = String::new();

        file.read_to_string(&mut data)?;

        let lines = data.lines().map(Line::from).collect();

        let data = Rc::new(RefCell::new(lines));
        let path = Some(path.canonicalize()?);

        Ok(Self { data, path })
    }

    pub fn data_pointer(&self) -> Rc<RefCell<Vec<Line>>> {
        Rc::clone(&self.data)
    }

    pub fn data(&self) -> Ref<'_, Vec<Line>> {
        self.data.borrow()
    }

    pub fn newline(&self, position: &Position) {
        let mut lines = self.data.borrow_mut();

        if let Some(line) = lines.get_mut(position.y) {
            let new = line.split(position.x);

            lines.insert(position.y.saturating_add(1), new)
        }
    }

    pub fn insert(&mut self, position: &Position, character: char) {
        let mut lines = self.data.borrow_mut();

        if character == '\n' {
            self.newline(position);
        } else if let Some(line) = lines.get_mut(position.y) {
            line.insert(position.x, character);
        }
    }

    pub fn delete(&mut self, position: &Position) {
        let mut lines = self.data.borrow_mut();

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
}
