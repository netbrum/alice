use super::c_result;
use libc::{ioctl, winsize, STDIN_FILENO, TIOCGWINSZ};
use std::{io::Result, mem::MaybeUninit};

#[derive(Debug)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

impl Size {
    pub fn new(height: u16, width: u16) -> Self {
        Size { height, width }
    }
}

pub fn get_terminal_size() -> Result<Size> {
    unsafe {
        let mut size: MaybeUninit<winsize> = MaybeUninit::uninit();
        c_result(ioctl(STDIN_FILENO, TIOCGWINSZ, size.as_mut_ptr()))?;
        let size = size.assume_init();

        Ok(Size {
            height: size.ws_row,
            width: size.ws_col,
        })
    }
}

pub fn get_terminal_size_pixels() -> Result<Size> {
    unsafe {
        let mut size: MaybeUninit<winsize> = MaybeUninit::uninit();
        c_result(ioctl(STDIN_FILENO, TIOCGWINSZ, size.as_mut_ptr()))?;
        let size = size.assume_init();

        Ok(Size {
            height: size.ws_ypixel,
            width: size.ws_xpixel,
        })
    }
}
