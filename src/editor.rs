mod mode;

use mode::Mode;

use super::buffer::{AlternateBuffer, IntoAlternateBuffer};
use super::event::Key;
use super::input::EventIterator;
use super::raw::{IntoRawMode, RawTerminal};

use std::{
    env::Args,
    io::{self, Result, Stdout},
};

pub struct Editor {
    mode: Mode,
    _out: RawTerminal<AlternateBuffer<Stdout>>,
}

impl Editor {
    pub fn new(_args: Args) -> Result<Self> {
        let _out = io::stdout().into_alternate_buffer()?.into_raw_mode()?;

        Ok(Editor {
            mode: Mode::Normal,
            _out,
        })
    }

    pub fn run(&mut self) {
        let mut stdin = io::stdin().lock().keys();

        loop {
            if self.mode == Mode::Exit {
                break;
            }

            if let Some(Ok(key)) = stdin.next() {
                self.handle_key(key);
            }
        }
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Ctrl('c') => self.mode = Mode::Exit,
            k => print!("{:?}\r\n", k),
        }
    }
}
