use std::{
    env,
    fs::{self, File},
    io::Result,
};

const LOG_FILE_NAME: &str = "log";
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn writer() -> Result<File> {
    let mut options = File::options();
    let options = options.append(true).create(true);

    if let Some(xdg) = env::var_os("XDG_STATE_HOME") {
        let xdg = xdg.to_str().unwrap();
        let path = format!("/{PACKAGE_NAME}/");

        fs::create_dir_all(xdg.to_string() + &path)?;

        options.open(xdg.to_string() + &path + LOG_FILE_NAME)
    } else if let Some(home) = env::var_os("HOME") {
        let home = home.to_str().unwrap();
        let path = format!("/.local/state/{PACKAGE_NAME}/");

        fs::create_dir_all(home.to_string() + &path)?;

        options.open(home.to_string() + &path + LOG_FILE_NAME)
    } else {
        File::options().append(true).open("/dev/stdout")
    }
}
