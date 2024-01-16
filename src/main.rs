use clap::Parser;

use alice::arg::Args;
use alice::editor::Editor;

fn main() {
    let args = Args::parse();
    let editor = Editor::new(args);

    match editor {
        Ok(mut e) => e.run(),
        Err(err) => {
            eprintln!("alice: {}", err.kind());
        }
    }
}
