mod buffer;
mod mode;
mod position;
mod status;
mod terminal;

use super::arg::Args;
use super::escape;
use super::event::Key;
use super::input::EventIterator;

use buffer::{cursor::Direction, line::Line, Buffer};
use mode::Mode;
use position::Position;
use status::Status;
use terminal::Terminal;

use std::io::{self, Result};

pub struct Editor {
    terminal: Terminal,
    buffer: Buffer,
    mode: Mode,
}

impl Editor {
    pub fn new(args: Args) -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new()?,
            buffer: Buffer::from_file(&args.path)?,
            mode: Mode::Normal,
        })
    }

    fn draw_line(&self, line: &Line) {
        let start = self.buffer.cursor.offset.x;
        let width = self.terminal.size.width as usize;
        let end = width + start;

        let line = line.render(start, end);
        print!("{line}\r\n");
    }

    fn draw(&self) {
        print!("{}", escape::cursor::RESET);

        let height = self.terminal.size.height as usize;
        let offset = self.buffer.cursor.offset.y;

        let lines = self.buffer.data();

        for index in 0..height {
            print!("{}", escape::clear::ENTIRE_LINE);

            if let Some(line) = lines.get(offset + index) {
                self.draw_line(line);
            }
        }

        Status::draw(self);
    }

    fn initial_draw(&self) {
        self.draw();

        print!("{}", escape::cursor::RESET);
        Terminal::flush();
    }

    fn redraw(&self) {
        self.draw();

        print!("{}", self.buffer.cursor);
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

            self.buffer.cursor.overstep(&self.mode);
            self.buffer.cursor.scroll(&self.terminal.size);

            self.redraw();
        }
    }

    fn handle_key_insert(&mut self, key: Key) {
        match key {
            Key::ArrowLeft | Key::ArrowDown | Key::ArrowUp | Key::ArrowRight => {
                self.buffer.cursor.step(Direction::from(key))
            }
            Key::Escape => {
                self.mode = Mode::Normal;
                print!("{}", escape::cursor::BLINKING_BLOCK);
                self.buffer.cursor.step(Direction::Left);
            }
            Key::Char(character) => {
                self.buffer.insert(character);
                self.buffer.cursor.step(Direction::Right);
            }
            Key::Enter => {
                self.buffer.newline();
                self.buffer.cursor.step(Direction::Down);
                self.buffer.cursor.position.x = 0;
            }
            Key::Backspace => {
                let Position { x, y } = self.buffer.cursor.position;

                if x > 0 || y > 0 {
                    self.buffer.cursor.backspace();
                    self.buffer.delete();
                }
            }
            _ => {}
        }
    }

    fn handle_key_normal(&mut self, key: Key) {
        match key {
            Key::Ctrl('c') => self.mode = Mode::Exit,
            Key::Char('h' | 'j' | 'k' | 'l')
            | Key::ArrowLeft
            | Key::ArrowDown
            | Key::ArrowUp
            | Key::ArrowRight => {
                self.buffer.cursor.step(Direction::from(key));
            }
            Key::Char('i') => {
                self.mode = Mode::Insert;
                print!("{}", escape::cursor::BLINKING_BAR);
                self.buffer.cursor.step(Direction::Right);
            }
            _ => {}
        }
    }

    fn handle_key(&mut self, key: Key) {
        match self.mode {
            Mode::Exit => unreachable!(),
            Mode::Normal => self.handle_key_normal(key),
            Mode::Insert => self.handle_key_insert(key),
        }
    }
}
