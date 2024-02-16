use super::Key;

use std::fmt::Display;

#[derive(Default)]
pub struct Command(pub Vec<Key>);

impl Command {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn insert(&mut self, key: Key) {
        self.0.push(key);
    }

    pub fn delete(&mut self) {
        self.0.pop();
    }

    pub fn keys(&mut self) -> &[Key] {
        &self.0[..]
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys = self.0.iter().map(|key| key.to_string()).collect::<String>();

        write!(f, "{keys}")
    }
}
