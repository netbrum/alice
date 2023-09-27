use super::{c_result, Termios};
use libc::{cfmakeraw, tcgetattr, tcsetattr, STDIN_FILENO, TCSANOW};
use std::{io::Result, mem::MaybeUninit};

pub fn enable_raw_mode(termios: &mut Termios) {
    unsafe { cfmakeraw(termios) }
}

pub fn get_terminal_attr() -> Result<Termios> {
    unsafe {
        let mut termios = MaybeUninit::uninit();
        c_result(tcgetattr(STDIN_FILENO, termios.as_mut_ptr()))?;
        Ok(termios.assume_init())
    }
}

pub fn set_terminal_attr(termios: &Termios) -> Result<()> {
    unsafe { c_result(tcsetattr(STDIN_FILENO, TCSANOW, termios)).and(Ok(())) }
}
