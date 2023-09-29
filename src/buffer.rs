use std::io::{Result, Write};

const ENABLE_ALTERNATE_BUFFER: &[u8; 8] = b"\x1b[?1049h";
const DISABLE_ALTERNATE_BUFFER: &[u8; 8] = b"\x1b[?1049l";

pub struct AlternateBuffer<W: Write> {
    out: W,
}

impl<W: Write> Drop for AlternateBuffer<W> {
    fn drop(&mut self) {
        self.out
            .write_all(DISABLE_ALTERNATE_BUFFER)
            .expect("disabling alternate buffer");

        self.out.flush().expect("flushing DISABLE_ALTERNATE_BUFFER");
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
        self.write_all(ENABLE_ALTERNATE_BUFFER)?;
        self.flush()?;

        Ok(AlternateBuffer { out: self })
    }
}
