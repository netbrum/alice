use std::env;

use alice::editor::Editor;

fn main() {
    let editor = Editor::new(env::args());

    match editor {
        Ok(mut e) => e.run(),
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
