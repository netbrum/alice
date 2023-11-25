mod cursor;
mod mode;

use cursor::{Cursor, Direction};
use mode::Mode;

use super::buffer::{AlternateBuffer, IntoAlternateBuffer};
use super::event::Key;
use super::input::EventIterator;
use super::raw::{IntoRawMode, RawTerminal};
use super::system::size::{self, Size};

use std::{
    env::Args,
    io::{self, Result, Stdout, Write},
};

pub struct Editor {
    mode: Mode,
    _out: RawTerminal<AlternateBuffer<Stdout>>,
    cursor: Cursor,
    size: Size,
}

impl Editor {
    pub fn new(_args: Args) -> Result<Self> {
        let _out = io::stdout().into_alternate_buffer()?.into_raw_mode()?;
        let size = size::get_terminal_size()?;

        Ok(Editor {
            mode: Mode::Normal,
            _out,
            cursor: Cursor::default(),
            size,
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

            _ = io::stdout().flush();
        }
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Ctrl('c') => self.mode = Mode::Exit,
            Key::Char('k') => self.cursor.step(Direction::Up, &self.size),
            Key::Char('j') => self.cursor.step(Direction::Down, &self.size),
            Key::Char('h') => self.cursor.step(Direction::Left, &self.size),
            Key::Char('l') => self.cursor.step(Direction::Right, &self.size),
            k => print!("{:?}", k),
        }
    }
}
