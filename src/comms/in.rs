// src/comms/in.rs
//! Utilities for taking user input

/// # Description
/// Prompts the user to make a selection. Takes user input, trims it, and
/// converts it to a string.
///
/// See ``src/package/ambiguity.rs`` for example usage
#[macro_export]
macro_rules! select {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        use std::io::{self, Write};
        let mut input = String::new();

        print!("{}{}: \x1b[0m", CONFIG.message.prompt, format!($($arg)*));
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_string();
        input
    }};
}

pub(crate) use select;
