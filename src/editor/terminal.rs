use crate::buffer::{AlternateBuffer, IntoAlternateBuffer};
use crate::raw::{IntoRawMode, RawTerminal};
use crate::system::size::{self, Size};

use std::io::{self, Result, Stdout};

pub struct Terminal {
    pub size: Size,
    _out: RawTerminal<AlternateBuffer<Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let _out = io::stdout().into_alternate_buffer()?.into_raw_mode()?;
        let size = size::get_terminal_size()?;

        Ok(Terminal { size, _out })
    }
}
