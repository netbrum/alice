mod buffer;
mod mode;
mod position;
mod terminal;

use super::arg::Args;
use super::escape;
use super::event::Key;
use super::input::EventIterator;

use buffer::{cursor::Direction, line::Line, Buffer};
use mode::Mode;
use position::Position;
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
        print!("{line}\r");
    }

    fn draw(&self) {
        print!("{}", escape::cursor::Reset);

        let height = self.terminal.size.height as usize;
        let offset = self.buffer.cursor.offset.y;

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

        let position = &self.buffer.cursor.position;
        let offset = &self.buffer.cursor.offset;

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

            self.buffer.cursor.scroll(&self.terminal.size);
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
            | Key::ArrowRight => self.buffer.cursor.step(Direction::from(key)),
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
                    self.buffer.cursor.overstep();
                }
            }
            k => print!("{:?}", k),
        }
    }
}
