use super::c_result;
use libc::{ioctl, winsize, STDIN_FILENO, TIOCGWINSZ};
use std::{io::Result, mem::MaybeUninit};

// (columns, rows)
#[derive(Debug)]
pub struct Size(u16, u16);

impl Size {
    pub fn new(x: u16, y: u16) -> Self {
        Size(x, y)
    }
}

pub fn get_terminal_size() -> Result<Size> {
    unsafe {
        let mut size: MaybeUninit<winsize> = MaybeUninit::uninit();
        c_result(ioctl(STDIN_FILENO, TIOCGWINSZ, size.as_mut_ptr()))?;
        let size = size.assume_init();

        Ok(Size(size.ws_col, size.ws_row))
    }
}

pub fn get_terminal_size_pixels() -> Result<Size> {
    unsafe {
        let mut size: MaybeUninit<winsize> = MaybeUninit::uninit();
        c_result(ioctl(STDIN_FILENO, TIOCGWINSZ, size.as_mut_ptr()))?;
        let size = size.assume_init();

        Ok(Size(size.ws_xpixel, size.ws_ypixel))
    }
}
