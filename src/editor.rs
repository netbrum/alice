mod buffer;
mod command;
mod mode;
mod position;
mod status;
mod terminal;

use super::arg::Args;
use super::escape;
use super::event::Key;
use super::input::EventIterator;

use buffer::{cursor::Direction, line::Line, Buffer};
use command::Command;
use mode::Mode;
use position::Position;
use status::{message::Message, Status};
use terminal::Terminal;

use std::io::{self, Result};

pub struct Editor {
    terminal: Terminal,
    buffer: Buffer,
    mode: Mode,
    command: Command,
    status: Status,
}

impl Editor {
    pub fn new(args: Args) -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new()?,
            buffer: Buffer::from_file(&args.path)?,
            mode: Mode::Normal,
            command: Command::default(),
            status: Status::default(),
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

        self.status.draw(
            &self.terminal.size,
            &self.mode,
            &self.buffer.cursor.position,
            &self.command,
        );
    }

    fn initial_draw(&self) {
        self.draw();

        print!("{}", escape::cursor::RESET);
        Terminal::flush();
    }

    fn redraw(&self) {
        self.draw();

        if self.mode != Mode::Command {
            print!("{}", self.buffer.cursor);
        }

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

    fn command_handler(&mut self) {
        match self.command.keys() {
            [Key::Char('q')] => {
                return self.mode = Mode::Exit;
            }
            [Key::Char('w')] => match self.buffer.save() {
                Ok(bytes) => {
                    let message = format!("Wrote {bytes} bytes to file");
                    self.status.message = Some(Message::new(&message));
                }
                Err(error) => {
                    self.status.message = Some(Message::new_err(&error.to_string()));
                }
            },
            keys => {
                let command: String = keys.iter().map(|key| key.to_string()).collect();
                let not_found = format!("Not a command: {}", command);

                self.status.message = Some(Message::new_err(&not_found));
            }
        }

        self.command.clear();
        self.mode = Mode::Normal;
        print!("{}", escape::cursor::BLINKING_BLOCK);
    }

    fn handle_key_command(&mut self, key: Key) {
        match key {
            Key::Escape => {
                self.mode = Mode::Normal;
                print!("{}", escape::cursor::BLINKING_BLOCK);
                self.command.clear();
            }
            Key::Backspace => self.command.delete(),
            Key::Char(_) => self.command.insert(key),
            Key::Enter => self.command_handler(),
            _ => {}
        }
    }

    fn handle_key_normal(&mut self, key: Key) {
        match key {
            Key::Char(' ') => {
                self.mode = Mode::Command;
                self.status.message = None;
                print!("{}", escape::cursor::BLINKING_BAR);
            }
            Key::Char('h' | 'j' | 'k' | 'l')
            | Key::ArrowLeft
            | Key::ArrowDown
            | Key::ArrowUp
            | Key::ArrowRight => {
                self.buffer.cursor.step(Direction::from(key));
            }
            Key::Char('g') => {
                self.buffer.cursor.top();
            }
            Key::Char('G') => {
                self.buffer.cursor.bottom();
            }
            Key::Char('f') => {
                self.buffer.cursor.start();
            }
            Key::Char('F') => {
                self.buffer.cursor.end();
            }
            Key::Char('C') => {
                self.buffer.cursor.center(&self.terminal.size);
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
            Mode::Command => self.handle_key_command(key),
        }
    }
}
