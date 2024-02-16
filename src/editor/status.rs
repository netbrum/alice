use super::{Editor, Mode, Position};

use crate::escape;
use crate::system::size::TermSize;

pub const RESERVED_HEIGHT: u16 = 2;

pub struct Status;

impl Status {
    fn draw_mode(mode: &Mode) {
        let background = match mode {
            Mode::Exit => escape::color::RED_BACKGROUND,
            Mode::Normal => escape::color::BRIGHT_GREEN_BACKGROUND,
            Mode::Insert => escape::color::BRIGHT_WHITE_BACKGROUND,
            Mode::Command => escape::color::YELLOW_BACKGROUND,
        };

        print!("{}", background);
        print!("{}", escape::color::BLACK_FOREGROUND);

        print!(" {} ", mode.to_string().to_uppercase());
    }

    fn draw_position(size: &TermSize, position: &Position) {
        print!("{}", escape::color::DEFAULT_BACKGROUND);
        print!("{}", escape::color::BRIGHT_BLACK_FOREGROUND);

        let position = format!(
            " {}:{} ",
            position.y.saturating_add(1),
            position.x.saturating_add(1)
        );

        let goto = escape::cursor::Goto(
            size.height as usize,
            (size.width as usize).saturating_sub(position.len()),
        );

        print!("{goto}{position}");
    }

    pub fn draw(editor: &Editor) {
        let height = editor.terminal.size.height.saturating_add(1);
        let width = editor.terminal.size.width.saturating_add(1);

        print!("{}", escape::cursor::Goto(height as usize, 0));
        print!("{}", escape::clear::ENTIRE_LINE);

        Self::draw_mode(&editor.mode);
        Self::draw_position(&TermSize { height, width }, &editor.buffer.cursor.position);

        print!("{}", escape::color::RESET);

        print!(
            "{}",
            escape::cursor::Goto(height.saturating_add(1) as usize, 0)
        );
        print!("{}", escape::clear::ENTIRE_LINE);

        if editor.mode == Mode::Command {
            print!(":{}", editor.command);
        }
    }
}
