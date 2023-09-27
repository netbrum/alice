pub mod attr;
pub mod size;
pub mod tty;

pub use libc::termios as Termios;

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
