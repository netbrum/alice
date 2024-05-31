mod buffer;
mod command;
mod mode;
mod position;
mod status;
mod terminal;
mod utils;

use super::arg::Args;
use super::escape;
use super::event::Key;
use super::input::EventIterator;

use crate::escape::CSI;
use buffer::{cursor::Direction, line::Line, Buffer};
use command::Command;
use mode::Mode;
use position::Position;
use status::{message::Message, Status};
use terminal::Terminal;

use std::fmt::Write;
use std::io::{self, Result};

const LINE_NUMBER_COLUMN_GAP: usize = 1;
const TAB_SIZE: usize = 2;

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

    fn line_number(&self, number: usize) -> String {
        let indent = utils::digits(self.buffer.data().len());
        let digits = utils::digits(number);

        format!(
            "{}{}{number}{} ",
            " ".repeat(indent - digits),
            escape::color::BRIGHT_BLACK_FOREGROUND,
            escape::color::RESET,
        )
    }

    fn highlight_line(
        &self,
        mut output: String,
        (index, char): (usize, char),
        highlights: &Option<Vec<CSI>>,
    ) -> String {
        if let Some(highlights) = &highlights {
            _ = write!(
                output,
                "{}",
                highlights[index + self.buffer.cursor.offset.x]
            );
        }

        _ = write!(output, "{char}");

        output
    }

    fn draw_line(&self, line: &Line, index: usize) {
        let start = self.buffer.cursor.offset.x;
        let end = self.terminal.size.width as usize + start - utils::ln_offset(&self.buffer.data());

        let highlights = &line.highlights;
        let line = line.render(start, end);

        let data = line
            .chars()
            .enumerate()
            .fold(String::new(), |output, ic| {
                self.highlight_line(output, ic, highlights)
            })
            .replace('\t', &" ".repeat(TAB_SIZE));

        let ln = self.line_number(index + 1);
        print!("{ln}{data}\r\n");
    }

    fn draw(&self) {
        print!("{}", escape::cursor::RESET);

        let height = self.terminal.size.height as usize;
        let offset = self.buffer.cursor.offset.y;

        let lines = self.buffer.data();

        for index in 0..height {
            print!("{}", escape::clear::ENTIRE_LINE);

            if let Some(line) = lines.get(offset + index) {
                self.draw_line(line, offset + index);
            } else {
                println!();
            }
        }

        self.status.draw(
            &self.terminal.size,
            &self.mode,
            self.buffer.file_name(),
            &self.buffer.cursor.position,
            &self.command,
        );
    }

    fn initial_draw(&mut self) {
        self.buffer.regenerate_highlights();
        self.draw();

        let offset = utils::ln_offset(&self.buffer.data());

        print!("{}", escape::cursor::Goto(0, offset + 1));
        print!("{}", escape::cursor::BLINKING_BLOCK);
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
            self.buffer.regenerate_highlights();

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
            Key::Tab => {
                self.buffer.insert('\t');
                self.buffer.cursor.step(Direction::Right);
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

    fn handle_command(&mut self) -> Option<Message> {
        match self.command.keys() {
            [Key::Char('q')] => {
                self.mode = Mode::Exit;
                None
            }
            [Key::Char('w')] => match self.buffer.save() {
                Ok(bytes) => {
                    let message = format!("Wrote {bytes} bytes to file");
                    Some(Message::new(&message))
                }
                Err(error) => Some(Message::new_err(&error.to_string())),
            },
            keys => {
                let command: String = keys.iter().map(|key| key.to_string()).collect();
                let not_found = format!("Not a command: {}", command);

                Some(Message::new_err(&not_found))
            }
        }
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
            Key::Enter => {
                self.status.message = self.handle_command();

                if self.mode != Mode::Exit {
                    self.command.clear();
                    self.mode = Mode::Normal;
                    print!("{}", escape::cursor::BLINKING_BLOCK);
                }
            }
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
