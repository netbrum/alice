mod buffer;
mod cursor;
mod mode;
mod position;
mod terminal;

use super::arg::Args;
use super::escape;
use super::event::Key;
use super::input::EventIterator;

use buffer::{line::Line, Buffer};
use cursor::{Cursor, Direction};
use mode::Mode;
use position::Position;
use terminal::Terminal;

use std::io::{self, Result};

pub struct Editor {
    terminal: Terminal,
    buffer: Buffer,
    cursor: Cursor,
    mode: Mode,
}

impl Editor {
    pub fn new(args: Args) -> Result<Self> {
        let buffer = Buffer::from_file(&args.path)?;
        let cursor = Cursor::new(buffer.data_pointer());

        Ok(Self {
            terminal: Terminal::new()?,
            buffer,
            cursor,
            mode: Mode::Normal,
        })
    }

    fn draw_line(&self, line: &Line) {
        let start = self.cursor.offset.x;
        let width = self.terminal.size.width as usize;
        let end = width + self.cursor.offset.x;
        let line = line.render(start, end);

        print!("{}\r", line);
    }

    fn draw(&self) {
        print!("{}", escape::cursor::Reset);

        let height = self.terminal.size.height as usize;
        let offset = self.cursor.offset.y;

        let lines = self.buffer.data();

        for index in 0..height {
            print!("{}", escape::clear::EntireLine);

            if let Some(line) = lines.get(offset + index) {
                self.draw_line(line);

                if index != height - 1 {
                    println!();
                }
            }
        }
    }

    fn initial_draw(&self) {
        self.draw();

        print!("{}", escape::cursor::Reset);
        Terminal::flush();
    }

    fn redraw(&self) {
        self.draw();

        let position = &self.cursor.position;
        let offset = &self.cursor.offset;

        let y = position.y.saturating_sub(offset.y).saturating_add(1);
        let x = position.x.saturating_sub(offset.x).saturating_add(1);

        print!("{}", escape::cursor::Goto(y, x));
        Terminal::flush();
    }

    pub fn run(&mut self) {
        self.initial_draw();

        let mut keys = io::stdin().lock().keys();

        loop {
            if self.mode == Mode::Exit {
                break;
            }

            if let Some(Ok(key)) = keys.next() {
                self.handle_key(key);
            }

            self.cursor.scroll(&self.terminal.size);
            self.redraw();
        }
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Ctrl('c') => self.mode = Mode::Exit,
            Key::Char('h' | 'j' | 'k' | 'l')
            | Key::ArrowLeft
            | Key::ArrowDown
            | Key::ArrowUp
            | Key::ArrowRight => self.cursor.step(Direction::from(key)),
            Key::Char(character) => {
                self.buffer.insert(&self.cursor.position, character);
                self.cursor.step(Direction::Right);
            }
            Key::Enter => {
                self.buffer.newline(&self.cursor.position);
                self.cursor.step(Direction::Down);
                self.cursor.position.x = 0;
            }
            Key::Backspace => {
                let Position { x, y } = self.cursor.position;

                if x > 0 || y > 0 {
                    self.cursor.backspace();
                    self.buffer.delete(&self.cursor.position);
                    self.cursor.overstep();
                }
            }
            k => print!("{:?}", k),
        }
    }
}
