use std::time::{Duration, Instant};

const OLD_DURATION: Duration = Duration::from_secs(5);

#[derive(Clone, Copy)]
pub enum MessageVariant {
    Normal,
    Error,
}

#[derive(Clone)]
pub struct Message {
    pub data: String,
    pub variant: MessageVariant,
    time: Instant,
}

impl Message {
    pub fn new(data: &str) -> Self {
        let data = data.to_string();

        Self {
            data,
            variant: MessageVariant::Normal,
            time: Instant::now(),
        }
    }

    pub fn new_err(data: &str) -> Self {
        let data = data.to_string();

        Self {
            data,
            variant: MessageVariant::Error,
            time: Instant::now(),
        }
    }

    pub fn is_old(&self) -> bool {
        Instant::now() - self.time > OLD_DURATION
    }
}
