mod cursor;
mod document;
mod mode;
mod position;
mod terminal;

use cursor::{Cursor, Direction};
use document::{Document, Row};
use mode::Mode;
use terminal::Terminal;

use super::arg::Args;
use super::event::Key;
use super::input::EventIterator;

use std::io::{self, Result};

pub struct Editor {
    mode: Mode,
    terminal: Terminal,
    cursor: Cursor,
    document: Document,
}

impl Editor {
    pub fn new(args: Args) -> Result<Self> {
        let terminal = Terminal::new()?;
        let document = Document::open(&args.path)?;

        let editor = Editor {
            mode: Mode::Normal,
            terminal,
            cursor: Cursor::default(),
            document,
        };

        editor.initial_draw();

        Ok(editor)
    }

    fn draw_row(&self, row: &Row) {
        let start = self.cursor.offset.x;
        let width = self.terminal.size.width as usize;
        let end = width + self.cursor.offset.x;
        let row = row.render(start, end);

        print!("{}\r", row);
    }

    fn draw(&self) {
        let height = self.terminal.size.height as usize;

        print!("\x1b[H");

        for index in 0..height {
            print!("\x1b[2K");

            let row = self.document.rows.get(index + self.cursor.offset.y);

            if let Some(row) = row {
                self.draw_row(row);

                if index != height - 1 {
                    println!();
                }
            }
        }
    }

    fn initial_draw(&self) {
        self.draw();

        print!("\x1b[H");
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

        print!("\x1b[{};{}H", y, x);
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
            Key::Char('k' | 'j' | 'h' | 'l') => {
                let direction = match key {
                    Key::Char('k') => Direction::Up,
                    Key::Char('j') => Direction::Down,
                    Key::Char('h') => Direction::Left,
                    Key::Char('l') => Direction::Right,
                    _ => unreachable!(),
                };

                self.cursor.step(direction, &self.document);
            }
            Key::Char(character) => {
                self.document.insert(&self.cursor.position, character);
                self.cursor.step(Direction::Right, &self.document);
            }
            k => print!("{:?}", k),
        }
    }
}
