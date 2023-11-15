mod mode;

use mode::Mode;

use super::buffer::{AlternateBuffer, IntoAlternateBuffer};
use super::raw::{IntoRawMode, RawTerminal};

use std::{
    env::Args,
    io::{self, Result, Stdout},
};

pub struct Editor {
    mode: Mode,
    out: RawTerminal<AlternateBuffer<Stdout>>,
}

impl Editor {
    pub fn new(_args: Args) -> Result<Self> {
        let out = io::stdout().into_alternate_buffer()?.into_raw_mode()?;

        Ok(Editor {
            mode: Mode::Normal,
            out,
        })
    }
}
