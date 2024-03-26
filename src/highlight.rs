mod rust;

use crate::escape::{self, CSI};
use rust::Rust;
use std::{ffi::OsStr, path::Path};
use tree_sitter::QueryError;
use tree_sitter_highlight::{Error, HighlightConfiguration, HighlightEvent, Highlighter};

const DEFAULT_HIGHLIGHT_NAMES: [&str; 59] = [
    "variable",
    "variable.builtin",
    "variable.parameter",
    "variable.parameter.builtin",
    "variable.member",
    "constant",
    "constant.builtin",
    "constant.macro",
    "module",
    "module.builtin",
    "label",
    "string",
    "string.documentation",
    "string.regexp",
    "string.escape",
    "string.special",
    "string.special.symbol",
    "string.special.url",
    "string.special.path",
    "character",
    "character.special",
    "boolean",
    "number",
    "number.float",
    "type",
    "type.builtin",
    "type.definition",
    "type.qualifier",
    "attribute",
    "attribute.builtin",
    "property",
    "function",
    "function.builtin",
    "function.call",
    "function.macro",
    "function.method",
    "function.emthod.call",
    "constructor",
    "operator",
    "keyword",
    "keyword.coroutine",
    "keyword.function",
    "keyword.operator",
    "keyword.import",
    "keyword.storage",
    "keyword.type",
    "keyword.repeat",
    "keyword.return",
    "keyword.debug",
    "keyword.exception",
    "keyword.conditional",
    "keyword.conditional.ternary",
    "keyword.directive",
    "keyword.directive.define",
    "punctuation.delimiter",
    "punctuation.bracket",
    "punctuation.special",
    "comment",
    "comment.documentation",
];

pub trait HighlightConfig {
    fn config() -> Result<HighlightConfiguration, QueryError>;
}

pub fn colors(config: &HighlightConfiguration, source: &[u8]) -> Result<Vec<CSI>, Error> {
    let mut highlighter = Highlighter::new();

    let highlights = highlighter.highlight(config, source, None, |_| None)?;

    let mut colors = vec![escape::color::DEFAULT_FOREGROUND; source.len()];
    let mut color: Option<CSI> = None;

    for event in highlights.flatten() {
        match event {
            HighlightEvent::Source { start, end } => {
                if let Some(color) = color {
                    for c in colors.iter_mut().take(end).skip(start) {
                        *c = color;
                    }
                }
            }
            HighlightEvent::HighlightStart(highlight) => {
                color = Some(match highlight.0 {
                    2 => escape::color::DEFAULT_FOREGROUND,
                    11 => escape::color::GREEN_FOREGROUND,
                    24 => escape::color::CYAN_FOREGROUND,
                    6 | 31 => escape::color::BLUE_FOREGROUND,
                    35 | 25 => escape::color::BRIGHT_BLUE_FOREGROUND,
                    10 | 30 | 38 => escape::color::YELLOW_FOREGROUND,
                    39 => escape::color::MAGENTA_FOREGROUND,
                    1 | 34 => escape::color::RED_FOREGROUND,
                    37 => escape::color::BRIGHT_YELLOW_FOREGROUND,
                    28 | 54 | 55 | 57 => escape::color::BRIGHT_BLACK_FOREGROUND,
                    n => {
                        unimplemented!("highlight {n}");
                    }
                });
            }
            HighlightEvent::HighlightEnd => {
                color = None;
            }
        }
    }

    Ok(colors)
}

pub fn config(path: &Path) -> Option<HighlightConfiguration> {
    let extension = path.extension().and_then(OsStr::to_str)?;

    match extension {
        "rs" => Rust::config().ok(),
        _ => None,
    }
}
