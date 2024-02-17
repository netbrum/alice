use std::time::{Duration, Instant};

const OLD_DURATION: Duration = Duration::from_secs(5);

#[derive(Clone)]
pub struct Message {
    pub data: String,
    time: Instant,
}

impl Message {
    pub fn new(data: &str) -> Self {
        let data = data.to_string();

        Self {
            data,
            time: Instant::now(),
        }
    }

    pub fn is_old(&self) -> bool {
        Instant::now() - self.time > OLD_DURATION
    }
}
