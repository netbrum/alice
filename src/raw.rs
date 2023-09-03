use super::system::attr;
use std::io::{Result, Write};

pub struct RawTerminal<W: Write> {
    previous_termios: libc::termios,
    out: W,
}

impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        let _ = attr::set_terminal_attr(&self.previous_termios);
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

pub trait RawMode: Write + Sized {
    fn raw_mode(self) -> Result<RawTerminal<Self>>;
}

impl<W: Write> RawMode for W {
    fn raw_mode(self) -> Result<RawTerminal<W>> {
        let mut termios = attr::get_terminal_attr()?;
        let previous = termios.clone();

        attr::enable_raw_mode(&mut termios);
        attr::set_terminal_attr(&mut termios)?;

        Ok(RawTerminal {
            previous_termios: previous,
            out: self,
        })
    }
}
