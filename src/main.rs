use alice::editor::Editor;
use alice::escape;
use alice::{arg::Args, system::attr};

use clap::Parser;

use std::io::{self, Write};
use std::panic;

fn main() {
    setup_panic_hook();

    let args = Args::parse();
    let editor = Editor::new(args);

    match editor {
        Ok(mut e) => e.run(),
        Err(err) => {
            eprintln!("alice: {}", err.kind());
        }
    }
}

fn setup_panic_hook() {
    if let Ok(termios) = attr::get_terminal_attr() {
        let default = panic::take_hook();

        panic::set_hook(Box::new(move |info| {
            let mut stdout = io::stdout();

            _ = attr::set_terminal_attr(&termios);

            let disable = escape::alternate::DISABLE.to_string();
            _ = stdout.write_all(disable.as_bytes());
            _ = stdout.flush();

            default(info);
        }));
    }
}
