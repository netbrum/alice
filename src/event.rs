use super::system::size::Size;

use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
};

const CTRL_1_OFFSET: u8 = 0x60;
const CTRL_2_OFFSET: u8 = 0x34;

#[derive(Debug)]
pub enum Event {
    Key(Key),
    Mouse(Size),
    Unknown(Vec<u8>),
}

#[derive(Debug)]
pub enum Key {
    Escape,
    Insert,
    Delete,
    Home,
    End,
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    PageUp,
    PageDown,
    Tab,
    Enter,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Null,
    Alt(char),
    Ctrl(char),
    Char(char),
}

pub fn parse_event<T>(byte: u8, iter: &mut T) -> Result<Event, Error>
where
    T: Iterator<Item = Result<u8, Error>> + Debug,
{
    let error = Error::new(ErrorKind::Other, "Could not parse event");

    match byte {
        // ESC sequence
        b'\x1b' => {
            let byte = iter.next();

            match byte {
                // CSI sequence
                Some(Ok(b'[')) => unimplemented!("csi handling"),
                Some(Ok(b'O')) => {
                    let byte = iter.next();
                    match byte {
                        Some(Ok(b'P')) => Ok(Event::Key(Key::F1)),
                        Some(Ok(b'Q')) => Ok(Event::Key(Key::F2)),
                        Some(Ok(b'R')) => Ok(Event::Key(Key::F3)),
                        Some(Ok(b'S')) => Ok(Event::Key(Key::F4)),
                        _ => Err(error),
                    }
                }
                Some(Ok(c)) => {
                    let utf8_char = parse_utf8(c, iter)?;
                    Ok(Event::Key(Key::Alt(utf8_char)))
                }
                _ => Err(error),
            }
        }
        b'\t' => Ok(Event::Key(Key::Tab)),
        b'\x7f' => Ok(Event::Key(Key::Backspace)),
        b'\n' | b'\r' => Ok(Event::Key(Key::Enter)),
        // Ctrl codes (a-z/1-26)
        c @ b'\x01'..=b'\x1a' => Ok(Event::Key(Key::Ctrl((c + CTRL_1_OFFSET) as char))),
        // Ctrl codes (3-7)
        c @ b'\x1c'..=b'\x1f' => Ok(Event::Key(Key::Ctrl((c + CTRL_2_OFFSET) as char))),
        b'\0' => Ok(Event::Key(Key::Null)),
        c => {
            let utf8_char = parse_utf8(c, iter)?;
            Ok(Event::Key(Key::Char(utf8_char)))
        }
    }
}

fn parse_utf8<T>(byte: u8, iter: &mut T) -> Result<char, Error>
where
    T: Iterator<Item = Result<u8, Error>>,
{
    if byte.is_ascii() {
        return Ok(byte as char);
    }

    let mut bytes = vec![byte];

    for byte in iter {
        match byte {
            Ok(byte) => {
                bytes.push(byte);

                if let Ok(utf8) = String::from_utf8(bytes.clone()) {
                    return Ok(utf8.chars().next().unwrap());
                }
            }
            Err(_) => return Err(Error::new(ErrorKind::Other, "Character is not valid UTF-8")),
        }
    }

    Err(Error::new(ErrorKind::Other, "Character is not valid UTF-8"))
}
