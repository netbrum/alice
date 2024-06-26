use crate::escape::CSI;

use std::fmt::Display;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Line {
    pub highlights: Option<Vec<CSI>>,
    data: String,
    length: usize,
}

impl Line {
    pub fn new(data: String) -> Self {
        let length = data.graphemes(true).count();

        Self {
            data,
            length,
            highlights: None,
        }
    }

    pub fn render(&self, start: usize, end: usize) -> String {
        let end = end.min(self.data.len());
        let start = start.min(end);

        self.data[start..end].replace('\t', " ")
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn update(&mut self) {
        let length = self.data.graphemes(true).count();
        self.length = length;
    }

    pub fn append(&mut self, data: &str) {
        self.data.push_str(data);
        self.update();
    }

    pub fn split(&mut self, x: usize) -> Self {
        let start: String = self.data.graphemes(true).take(x).collect();
        let end: String = self.data.graphemes(true).skip(x).collect();

        self.data = start;
        self.update();

        Self::from(end.as_str())
    }

    pub fn insert(&mut self, x: usize, character: char) {
        let start: String = self.data.graphemes(true).take(x).collect();
        let end: String = self.data.graphemes(true).skip(x).collect();

        let new = format!("{start}{character}{end}");

        self.data = new;
        self.update();
    }

    pub fn delete(&mut self, x: usize) {
        let start: String = self.data.graphemes(true).take(x).collect();
        let end: String = self.data.graphemes(true).skip(x + 1).collect();

        self.data = format!("{start}{end}");
        self.update();
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

impl From<&str> for Line {
    fn from(data: &str) -> Self {
        Self::new(data.to_owned())
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
