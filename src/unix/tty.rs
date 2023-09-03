use std::{
    fs::{File, OpenOptions},
    io::Result,
};

pub fn tty() -> Result<File> {
    OpenOptions::new().write(false).read(true).open("/dev/tty")
}
