pub mod attr;
pub mod size;

use std::io::{Error, Result};

trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

impl IsMinusOne for i32 {
    fn is_minus_one(&self) -> bool {
        *self == -1
    }
}

fn c_result<T: IsMinusOne>(n: T) -> Result<T> {
    if n.is_minus_one() {
        Err(Error::last_os_error())
    } else {
        Ok(n)
    }
}

pub fn setup_terminal() -> Result<libc::termios> {
    let mut termios = attr::get_terminal_attr()?;
    attr::enable_raw_mode(&mut termios);

    attr::set_terminal_attr(&mut termios)?;

    Ok(termios)
}
