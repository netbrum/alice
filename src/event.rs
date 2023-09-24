use super::system::size::Size;

use std::io::{Error, ErrorKind, Result};

const CTRL_1_OFFSET: u8 = 0x60;
const CTRL_2_OFFSET: u8 = 0x34;

const PARSE_ERROR: &str = "Could not parse event";
const UTF8_ERROR: &str = "Character is not valid UTF-8";

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

pub fn parse_event<T>(byte: u8, iter: &mut T) -> Result<Event>
where
    T: Iterator<Item = Result<u8>>,
{
    match byte {
        // An Escape sequence
        //
        // The first byte in an escape sequence is always:
        // 0x1b (hexadecimal) / 27 (decimal)
        b'\x1b' => {
            let byte = iter.next();

            // If there is no next byte, we can assume it's the Escape key
            if byte.is_none() {
                return Ok(Event::Key(Key::Escape));
            }

            let byte = byte.ok_or(Error::new(ErrorKind::Other, PARSE_ERROR))?;

            match byte {
                // A CSI (Control Sequence Introducer)
                //
                // The first two bytes in a CSI is always:
                // 1: 0x1b (hexadecimal) / 27 (decimal)
                // 2: 0x5b (hexadecimal) / 91 (decimal)
                Ok(b'[') => parse_csi(iter).ok_or(Error::new(ErrorKind::Other, PARSE_ERROR)),
                Ok(b'O') => {
                    let byte = iter
                        .next()
                        .ok_or(Error::new(ErrorKind::Other, PARSE_ERROR))?;

                    match byte {
                        Ok(b'P') => Ok(Event::Key(Key::F1)),
                        Ok(b'Q') => Ok(Event::Key(Key::F2)),
                        Ok(b'R') => Ok(Event::Key(Key::F3)),
                        Ok(b'S') => Ok(Event::Key(Key::F4)),
                        _ => Err(Error::new(ErrorKind::Other, PARSE_ERROR)),
                    }
                }
                Ok(c) => {
                    let utf8_char = parse_utf8(c, iter)?;
                    Ok(Event::Key(Key::Alt(utf8_char)))
                }
                _ => Err(Error::new(ErrorKind::Other, PARSE_ERROR)),
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

fn parse_csi<T>(iter: &mut T) -> Option<Event>
where
    T: Iterator<Item = Result<u8>>,
{
    let byte = iter.next()?.ok()?;

    match byte {
        b'A' => Some(Event::Key(Key::ArrowUp)),
        b'B' => Some(Event::Key(Key::ArrowDown)),
        b'C' => Some(Event::Key(Key::ArrowRight)),
        b'D' => Some(Event::Key(Key::ArrowLeft)),
        b'F' => Some(Event::Key(Key::End)),
        b'H' => Some(Event::Key(Key::Home)),
        b'0'..=b'9' => {
            let mut buf = vec![byte];

            let mut byte = iter.next()?.ok()?;
            buf.push(byte);

            // The last value of a CSI is always gonna be in the range of 64-126
            while byte < 64 || byte > 126 {
                byte = iter.next()?.ok()?;
                buf.push(byte);
            }

            let byte = buf.pop()?;

            match byte {
                b'~' => {
                    let buf = String::from_utf8(buf).ok()?;

                    match buf.as_str() {
                        "1" | "7" => Some(Event::Key(Key::Home)),
                        "2" => Some(Event::Key(Key::Insert)),
                        "3" => Some(Event::Key(Key::Delete)),
                        "4" | "8" => Some(Event::Key(Key::End)),
                        "5" => Some(Event::Key(Key::PageUp)),
                        "6" => Some(Event::Key(Key::PageDown)),
                        "15" => Some(Event::Key(Key::F5)),
                        "17" => Some(Event::Key(Key::F6)),
                        "18" => Some(Event::Key(Key::F7)),
                        "19" => Some(Event::Key(Key::F8)),
                        "20" => Some(Event::Key(Key::F9)),
                        "21" => Some(Event::Key(Key::F10)),
                        "23" => Some(Event::Key(Key::F11)),
                        "24" => Some(Event::Key(Key::F12)),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn parse_utf8<T>(byte: u8, iter: &mut T) -> Result<char>
where
    T: Iterator<Item = Result<u8>>,
{
    if byte.is_ascii() {
        return Ok(byte as char);
    }

    let mut bytes = vec![byte];

    for byte in iter {
        match byte {
            Ok(byte) => {
                // Keep pushing a byte to the 'bytes' vector until we can build a valid UTF-8
                // string from the vector
                bytes.push(byte);

                if let Ok(utf8) = String::from_utf8(bytes.clone()) {
                    // A UTF-8 string can be built from an empty vector, in which case it just
                    // returns an empty string, which would panic if we try to use chars().next().unwrap()
                    //
                    // But as we create the 'bytes' vector with an initial value, we know there is at
                    // minimum 1 value in the vector, so we can safely use unwrap() here
                    return Ok(utf8.chars().next().unwrap());
                }
            }
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, UTF8_ERROR)),
        }
    }

    Err(Error::new(ErrorKind::InvalidData, UTF8_ERROR))
}
