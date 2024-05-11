use alice::{
    arg::Args,
    editor::Editor,
    escape,
    system::{attr, log},
};

use clap::Parser;

use std::io::{self, Write};
use std::panic;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
    let writer = log::writer().expect("getting output file failed");
    let (non_blocking, _guard) = tracing_appender::non_blocking(writer);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .with_ansi(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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
