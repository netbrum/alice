use super::system::tty;

use std::{
    io::{Read, Result},
    sync::mpsc::{self, channel},
    thread,
};

// Input reader for tty, as it runs in a separate thread it won't block
pub struct TTYReader {
    reciever: mpsc::Receiver<Result<u8>>,
}

impl TTYReader {
    pub fn new() -> Self {
        let (sender, reciever) = channel();

        thread::spawn(move || {
            for byte in tty::tty().expect("to read /dev/tty").bytes() {
                if sender.send(byte).is_err() {
                    return;
                }
            }
        });

        TTYReader { reciever }
    }
}

impl Read for TTYReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut total = 0;

        while total < buf.len() {
            match self.reciever.try_recv() {
                Ok(byte) => {
                    let byte = byte?;

                    buf[total] = byte;
                    total += 1;
                }
                Err(_) => break,
            }
        }

        Ok(total)
    }
}
