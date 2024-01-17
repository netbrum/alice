use super::escape;

use std::io::{Result, Write};

pub struct AlternateBuffer<W: Write> {
    out: W,
}

impl<W: Write> Drop for AlternateBuffer<W> {
    fn drop(&mut self) {
        let disable = escape::alternate::Disable.to_string();

        _ = self.out.write_all(disable.as_bytes());
        _ = self.out.flush();
    }
}

impl<W: Write> Write for AlternateBuffer<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.out.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.out.flush()
    }
}

pub trait IntoAlternateBuffer: Write + Sized {
    fn into_alternate_buffer(self) -> Result<AlternateBuffer<Self>>;
}

impl<W: Write> IntoAlternateBuffer for W {
    fn into_alternate_buffer(mut self) -> Result<AlternateBuffer<Self>> {
        let enable = escape::alternate::Enable.to_string();

        self.write_all(enable.as_bytes())?;
        self.flush()?;

        Ok(AlternateBuffer { out: self })
    }
}
