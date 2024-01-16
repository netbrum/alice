use super::c_result;
use libc::{ioctl, winsize, STDIN_FILENO, TIOCGWINSZ};
use std::{io::Result, mem::MaybeUninit};

#[derive(Debug)]
pub struct TermSize {
    pub height: u16,
    pub width: u16,
}

pub fn get_terminal_size() -> Result<TermSize> {
    unsafe {
        let mut size: MaybeUninit<winsize> = MaybeUninit::uninit();
        c_result(ioctl(STDIN_FILENO, TIOCGWINSZ, size.as_mut_ptr()))?;
        let size = size.assume_init();

        Ok(TermSize {
            height: size.ws_row,
            width: size.ws_col,
        })
    }
}

pub fn get_terminal_size_pixels() -> Result<TermSize> {
    unsafe {
        let mut size: MaybeUninit<winsize> = MaybeUninit::uninit();
        c_result(ioctl(STDIN_FILENO, TIOCGWINSZ, size.as_mut_ptr()))?;
        let size = size.assume_init();

        Ok(TermSize {
            height: size.ws_ypixel,
            width: size.ws_xpixel,
        })
    }
}
