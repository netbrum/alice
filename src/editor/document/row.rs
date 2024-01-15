use std::fmt::Display;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    buffer: String,
    length: usize,
}

impl Row {
    pub fn new(buffer: String) -> Self {
        let length = buffer.graphemes(true).count();

        Row { buffer, length }
    }

    pub fn render(&self, start: usize, end: usize) -> &str {
        let end = end.min(self.buffer.len());
        let start = start.min(end);

        &self.buffer[start..end]
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn update(&mut self) {
        let length = self.buffer.graphemes(true).count();
        self.length = length;
    }

    pub fn append(&mut self, buffer: &str) {
        self.buffer.push_str(buffer);
        self.update();
    }

    pub fn insert(&mut self, x: usize, character: char) {
        let start: String = self.buffer.graphemes(true).take(x).collect();
        let end: String = self.buffer.graphemes(true).skip(x).collect();

        let new = format!("{start}{character}{end}");

        self.buffer = new;
        self.update();
    }

    pub fn delete(&mut self, x: usize) {
        let start: String = self.buffer.graphemes(true).take(x).collect();
        let end: String = self.buffer.graphemes(true).skip(x + 1).collect();

        self.buffer = format!("{start}{end}");
        self.update();
    }
}

impl From<&str> for Row {
    fn from(buffer: &str) -> Self {
        Row::new(buffer.to_owned())
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buffer)
    }
}
