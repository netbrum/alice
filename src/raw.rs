use super::system::{attr, Termios};
use std::io::{Result, Write};

pub struct RawTerminal<W: Write> {
    previous_termios: Termios,
    out: W,
}

impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        _ = attr::set_terminal_attr(&self.previous_termios);
    }
}

impl<W: Write> Write for RawTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.out.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.out.flush()
    }
}

pub trait IntoRawMode: Write + Sized {
    fn into_raw_mode(self) -> Result<RawTerminal<Self>>;
}

impl<W: Write> IntoRawMode for W {
    fn into_raw_mode(self) -> Result<RawTerminal<W>> {
        let mut termios = attr::get_terminal_attr()?;
        let previous = termios;

        attr::enable_raw_mode(&mut termios);
        attr::set_terminal_attr(&termios)?;

        Ok(RawTerminal {
            previous_termios: previous,
            out: self,
        })
    }
}
