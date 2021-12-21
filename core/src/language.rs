use std::str::FromStr;
use std::{collections::HashMap, path::PathBuf};
use tree_sitter::Parser;
use tree_sitter_go;
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_html;
use tree_sitter_json;
use tree_sitter_ruby;
use tree_sitter_rust;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum LapceLanguage {
    Rust,
    Go,
    Ruby,
    Json,
    Html,
}

impl LapceLanguage {
    pub fn from_path(path: &PathBuf) -> Option<LapceLanguage> {
        let ext = path.extension();
        if ext == None {
            let file_name = path.file_stem()?.to_str()?;
            return match file_name {
                "Gemfile" | "Guardfile" | "Rakefile" | "gemspec" | "Procfile" => {
                    Some(LapceLanguage::Ruby)
                }
                _ => None,
            };
        }

        let extension = ext?.to_str()?;
        Some(match extension {
            "rs" => LapceLanguage::Rust,
            "go" => LapceLanguage::Go,
            "rb" | "rake" | "ru" => LapceLanguage::Ruby,
            "json" => LapceLanguage::Json,
            "html" => LapceLanguage::Html,
            _ => return None,
        })
    }
}

pub struct TreeSitter {
    parsers: HashMap<LapceLanguage, Parser>,
}

pub fn new_highlight_config(
    language: LapceLanguage,
) -> (HighlightConfiguration, Vec<String>) {
    match language {
        LapceLanguage::Rust => {
            let mut configuration = HighlightConfiguration::new(
                tree_sitter_rust::language(),
                tree_sitter_rust::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap();

            let recognized_names = vec![
                "constant",
                "constant.builtin",
                "type",
                "type.builtin",
                "property",
                "comment",
                "constructor",
                "function",
                "function.method",
                "function.macro",
                "punctuation.bracket",
                "punctuation.delimiter",
                "label",
                "keyword",
                "string",
                "variable.parameter",
                "variable.builtin",
                "operator",
                "attribute",
                "escape",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
            configuration.configure(&recognized_names);

            (configuration, recognized_names)
        }
        LapceLanguage::Go => {
            let mut configuration = HighlightConfiguration::new(
                tree_sitter_go::language(),
                tree_sitter_go::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap();
            let recognized_names = vec![
                "constant",
                "constant.builtin",
                "type",
                "type.builtin",
                "property",
                "comment",
                "constructor",
                "function",
                "function.method",
                "function.macro",
                "punctuation.bracket",
                "punctuation.delimiter",
                "label",
                "keyword",
                "string",
                "variable.parameter",
                "variable.builtin",
                "operator",
                "attribute",
                "escape",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
            configuration.configure(&recognized_names);

            (configuration, recognized_names)
        }
        LapceLanguage::Ruby => {
            let mut configuration = HighlightConfiguration::new(
                tree_sitter_ruby::language(),
                tree_sitter_ruby::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap();
            let recognized_names = vec![
                "constant",
                "constant.builtin",
                "type",
                "type.builtin",
                "property",
                "comment",
                "constructor",
                "function",
                "function.method",
                "function.macro",
                "punctuation.bracket",
                "punctuation.delimiter",
                "label",
                "keyword",
                "string",
                "variable.parameter",
                "variable.builtin",
                "operator",
                "attribute",
                "escape",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
            configuration.configure(&recognized_names);

            (configuration, recognized_names)
        }
        LapceLanguage::Html => {
            let mut configuration = HighlightConfiguration::new(
                tree_sitter_html::language(),
                tree_sitter_html::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap();
            let recognized_names = vec![
                "constant",
                "constant.builtin",
                "type",
                "type.builtin",
                "property",
                "comment",
                "constructor",
                "function",
                "function.method",
                "function.macro",
                "punctuation.bracket",
                "punctuation.delimiter",
                "label",
                "keyword",
                "string",
                "variable.parameter",
                "variable.builtin",
                "operator",
                "attribute",
                "escape",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
            configuration.configure(&recognized_names);

            (configuration, recognized_names)
        }
        LapceLanguage::Json => {
            let mut configuration = HighlightConfiguration::new(
                tree_sitter_json::language(),
                tree_sitter_json::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap();
            let recognized_names = vec![
                "punctuation.bracket",
                "punctuation.delimiter",
                "keyword",
                "string",
                "escape",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
            configuration.configure(&recognized_names);

            (configuration, recognized_names)
        }
    }
}

pub fn new_parser(language: LapceLanguage) -> Parser {
    let language = match language {
        LapceLanguage::Rust => tree_sitter_rust::language(),
        LapceLanguage::Go => tree_sitter_go::language(),
        LapceLanguage::Ruby => tree_sitter_ruby::language(),
        LapceLanguage::Json => tree_sitter_json::language(),
        LapceLanguage::Html => tree_sitter_html::language(),
    };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    parser
}

impl TreeSitter {
    pub fn new() -> TreeSitter {
        let mut parsers = HashMap::new();

        let mut parser = Parser::new();
        let language = tree_sitter_rust::language();
        parser.set_language(language);
        parsers.insert(LapceLanguage::Rust, parser);

        TreeSitter { parsers }
    }
}
