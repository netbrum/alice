mod buffer;
mod cursor;
mod mode;
mod position;
mod terminal;

use buffer::{Buffer, Line};
use cursor::{Cursor, Direction};
use mode::Mode;
use position::Position;
use terminal::Terminal;

use super::arg::Args;
use super::escape;
use super::event::Key;
use super::input::EventIterator;

use std::io::{self, Result};

pub struct Editor {
    mode: Mode,
    terminal: Terminal,
    cursor: Cursor,
    buffer: Buffer,
}

impl Editor {
    pub fn new(args: Args) -> Result<Self> {
        let terminal = Terminal::new()?;
        let buffer = Buffer::from_file(&args.path)?;

        let editor = Self {
            mode: Mode::Normal,
            terminal,
            cursor: Cursor::default(),
            buffer,
        };

        editor.initial_draw();

        Ok(editor)
    }

    fn draw_line(&self, line: &Line) {
        let start = self.cursor.offset.x;
        let width = self.terminal.size.width as usize;
        let end = width + self.cursor.offset.x;
        let line = line.render(start, end);

        print!("{}\r", line);
    }

    fn draw(&self) {
        let height = self.terminal.size.height as usize;

        print!("{}", escape::cursor::Reset);

        for index in 0..height {
            print!("{}", escape::clear::EntireLine);

            let line = self.buffer.lines.get(index + self.cursor.offset.y);

            if let Some(line) = line {
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

        let x = self
            .cursor
            .position
            .x
            .saturating_sub(self.cursor.offset.x)
            .saturating_add(1);

        let y = self
            .cursor
            .position
            .y
            .saturating_sub(self.cursor.offset.y)
            .saturating_add(1);

        print!("{}", escape::cursor::Goto(y, x));
        Terminal::flush();
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
            | Key::ArrowRight => self.cursor.step(Direction::from(key), &self.buffer),
            Key::Char(character) => {
                self.buffer.insert(&self.cursor.position, character);
                self.cursor.step(Direction::Right, &self.buffer);
            }
            Key::Enter => {
                self.buffer.newline(&self.cursor.position);
                self.cursor.step(Direction::Down, &self.buffer);
                self.cursor.position.x = 0;
            }
            Key::Backspace => {
                let Position { x, y } = self.cursor.position;

                if x > 0 || y > 0 {
                    self.cursor.backspace(&self.buffer);
                    self.buffer.delete(&self.cursor.position);
                    self.cursor.overstep(&self.buffer);
                }
            }
            k => print!("{:?}", k),
        }
    }
}
