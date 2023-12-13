mod cursor;
mod document;
mod mode;
mod terminal;

use cursor::{Cursor, Direction};
use document::{Document, Row};
use mode::Mode;
use terminal::Terminal;

use super::arg::Args;
use super::event::Key;
use super::input::EventIterator;
use super::system::size::Size;

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
        let end = self.terminal.size.width as usize;
        let row = row.render(0, end);

        print!("{}\r\n", row);
    }

    fn initial_draw(&self) {
        for row in &self.document.rows {
            self.draw_row(row);
        }

        print!("\x1b[H");

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

            Terminal::flush();
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

                self.cursor.step(direction, &self.terminal.size);

                // The cursor struct is 0 based, while the ANSI escape codes for the cursor is 1
                // based, so we transform the values before visually moving the cursor
                let x = self.cursor.x.saturating_add(1);
                let y = self.cursor.y.saturating_add(1);

                print!("\x1b[{};{}H", y, x);
            }
            k => print!("{:?}", k),
        }
    }
}
