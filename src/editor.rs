mod mode;

use mode::Mode;

use super::buffer::{AlternateBuffer, IntoAlternateBuffer};
use super::event::Key;
use super::input::EventIterator;
use super::raw::{IntoRawMode, RawTerminal};

use std::{
    env::Args,
    io::{self, Result, Stdout, Write},
};

pub struct Editor {
    mode: Mode,
    out: RawTerminal<AlternateBuffer<Stdout>>,
}

impl Editor {
    pub fn new(_args: Args) -> Result<Self> {
        let out = io::stdout().into_alternate_buffer()?.into_raw_mode()?;

        Ok(Editor {
            mode: Mode::Normal,
            out,
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

    fn print(&mut self, text: &str) -> Result<()> {
        self.out.write_all(text.as_bytes())?;
        self.out.flush()?;

        Ok(())
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Ctrl('c') => self.mode = Mode::Exit,
            k => {
                self.print(&format!("{:?}\r\n", k)).unwrap();
            }
        }
    }
}
