// src/comms/log.rs
//! Some utility functions for communicating with the user
//! All the macros in this module take formatting from the message config

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
