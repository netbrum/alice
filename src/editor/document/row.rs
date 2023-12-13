use std::fmt::Display;

#[derive(Default)]
pub struct Row {
    buffer: String,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Row {
            buffer: value.to_owned(),
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buffer)
    }
}
