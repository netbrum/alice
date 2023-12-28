use std::fmt::Display;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    buffer: String,
    length: usize,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> &str {
        let end = end.min(self.buffer.len());
        let start = start.min(end);

        &self.buffer[start..end]
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Row {
            buffer: value.to_owned(),
            length: value.graphemes(true).count(),
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buffer)
    }
}
