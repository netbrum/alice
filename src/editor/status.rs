use super::{Command, Mode, Position};

use crate::escape;
use crate::system::size::TermSize;

pub const RESERVED_HEIGHT: u16 = 2;

pub struct Status;

impl Status {
    fn draw_command(mode: &Mode, command: &Command) {
        if *mode == Mode::Command {
            print!(":{command}");
        }
    }

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
        print!("{}", escape::color::RESET);
    }

    fn draw_position(position: &Position, size: &TermSize) {
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
        print!("{}", escape::color::RESET);
    }

    pub fn draw(size: &TermSize, mode: &Mode, position: &Position, command: &Command) {
        let height = size.height.saturating_add(1);
        let width = size.width.saturating_add(1);

        let size = &TermSize { height, width };

        let goto = escape::cursor::Goto(size.height as usize, 0);
        print!("{goto}{}", escape::clear::ENTIRE_LINE);
        Self::draw_mode(mode);
        Self::draw_position(position, size);

        let goto = escape::cursor::Goto(size.height.saturating_add(1) as usize, 0);
        print!("{goto}{}", escape::clear::ENTIRE_LINE);
        Self::draw_command(mode, command);
    }
}
