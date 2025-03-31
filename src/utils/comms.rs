// src/comms/in.rs
//! Utilities for communicating with the user

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

/// # Description
/// Prints to stdout with the danger formatting
///
/// Unaffected by the quiet flag
#[macro_export]
macro_rules! erm {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        eprintln!("{}{}\x1b[0m", CONFIG.message.danger, format!($($arg)*))
    }};
}

/// # Description
/// Sends a message to stdout with the message formatting
///
/// Unaffected by the quiet flag
#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        println!("{}{}\x1b[0m", CONFIG.message.message, format!($($arg)*))
    }};
}

/// # Description
/// Prints to stdout with the default formatting
///
/// Unaffected by the quiet flag
#[macro_export]
macro_rules! pr {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        println!("{}{}\x1b[0m", CONFIG.message.default, format!($($arg)*))
    }};
}

/// # Description
/// Prints to stdout with the verbose formatting
///
/// Unaffected by the quiet flag, enabled by the verbose flag
#[macro_export]
#[cfg(not(test))]
macro_rules! vpr {
    ($($arg:tt)*) => {{
        use $crate::globals::flags::Flags;
        if Flags::grab().verbose {
            use $crate::globals::config::CONFIG;
            let f = std::path::Path::new(file!())
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            println!("{}[{}] {}\x1b[0m", CONFIG.message.verbose, f, format!($($arg)*))
        }
    }};
}

/// # Description
/// Prints to stdout with the verbose formatting
///
/// Unaffected by the quiet flag, enabled by the verbose flag
#[macro_export]
#[cfg(test)]
macro_rules! vpr {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        let f = std::path::Path::new(file!())
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");
        println!("{}[{}] {}\x1b[0m", CONFIG.message.verbose, f, format!($($arg)*))
    }};
}

pub(crate) use erm;
pub(crate) use msg;
pub(crate) use pr;
pub(crate) use vpr;
