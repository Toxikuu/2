// src/utils/esc.rs
//! Utilities related to escape codes

pub fn escape_escapes(string: &str) -> String {
    string
        .replace("\\x1b", "\x1b")
        .replace("\\e", "\x1b")
        .replace("\\u001b", "\u{001b}")
}
