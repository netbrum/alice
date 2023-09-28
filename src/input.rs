use super::event::{self, Event, Key};
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

impl Default for TTYReader {
    fn default() -> Self {
        Self::new()
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

pub struct RawEvents<R: Read> {
    source: R,
    // We need a remainder to handle pasting, as we read two bytes at a time in the iterator
    // implementation, without the remainder, you would get every second character when pasting
    //
    // For example, when pasting "rust", it advances the iterator four times, but due to reading
    // two bytes at a time, you would get the following:
    //
    // 1. [114, 117]
    // 2. Nothing
    // 3. [115, 116]
    // 4. Nothing
    //
    // When the last value isn't consumed, it is saved in the remainder for the next iteration
    remainder: Option<u8>,
}

impl<R: Read> Iterator for RawEvents<R> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        let source = &mut self.source;

        if let Some(remainder) = self.remainder {
            self.remainder = None;
            return Some(event::parse_event(remainder, &mut source.bytes()));
        }

        // Read two bytes so we can differentiate between an escape sequence and an escape key,
        // this can't be done in the event parser due to stdin blocking
        let mut buf = [0u8; 2];
        let read = source.read(&mut buf);

        match read {
            Ok(0) => None,
            Ok(1) => match buf[0] {
                b'\x1b' => Some(Ok(Event::Key(Key::Escape))),
                byte => Some(event::parse_event(byte, &mut source.bytes())),
            },
            Ok(2) => {
                let iter = &mut Some(buf[1]).into_iter();
                let event = {
                    let mut iter = iter.map(Ok).chain(source.bytes());
                    event::parse_event(buf[0], &mut iter)
                };

                self.remainder = iter.next();

                Some(event)
            }
            Err(e) => Some(Err(e)),
            _ => unreachable!(),
        }
    }
}

pub struct Keys<R: Read> {
    pub inner: RawEvents<R>,
}

impl<R: Read> Iterator for Keys<R> {
    type Item = Result<Key>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.inner.next()?.ok()?;

        match event {
            Event::Key(k) => Some(Ok(k)),
            _ => None,
        }
    }
}

pub trait EventIterator: Read + Sized {
    fn keys(self) -> Keys<Self>;
    fn events(self) -> RawEvents<Self>;
}

impl<R: Read> EventIterator for R {
    fn keys(self) -> Keys<Self> {
        let inner = RawEvents {
            source: self,
            remainder: None,
        };

        Keys { inner }
    }

    fn events(self) -> RawEvents<Self> {
        RawEvents {
            source: self,
            remainder: None,
        }
    }
}
