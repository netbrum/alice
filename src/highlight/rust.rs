use super::{HighlightConfig, HighlightConfiguration, QueryError, DEFAULT_HIGHLIGHT_NAMES};

pub struct Rust;

impl HighlightConfig for Rust {
    fn config() -> Result<HighlightConfiguration, QueryError> {
        let language = tree_sitter_rust::language();

        let mut config =
            HighlightConfiguration::new(language, tree_sitter_rust::HIGHLIGHT_QUERY, "", "")?;

        config.configure(&DEFAULT_HIGHLIGHT_NAMES);

        Ok(config)
    }
}
