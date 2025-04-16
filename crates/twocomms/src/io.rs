// twocomms/src/io.rs

/// # Description
/// Prompts the user to make a selection. Takes user input, trims it, and
/// converts it to a string.
///
/// See ``src/package/ambiguity.rs`` for example usage
#[macro_export]
macro_rules! select {
    ($($arg:tt)*) => {{
        use twoconfig::CONFIG;
        use std::io::{self, Write};
        use tracing::debug;

        let mut input = String::new();
        let prompt = format!("{}", format!($($arg)*));

        print!("{}{prompt}: \x1b[0m", CONFIG.message.prompt);
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        debug!("Received user input '{input}' from prompt '{prompt}'");
        input
    }};
}

/// # Description
/// Prints to stdout with the danger formatting
///
/// Unaffected by the quiet flag
#[macro_export]
macro_rules! erm {
    ($($arg:tt)*) => {{
        use twoconfig::CONFIG;
        use tracing::warn;

        let message = format!("{}", format!($($arg)*));
        eprintln!("{}{message}\x1b[0m", CONFIG.message.danger);
        warn!("Sent error message:\n\t{message}")
    }};
}

/// # Description
/// Sends a message to stdout with the message formatting
///
/// Unaffected by the quiet flag
#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {{
        use twoconfig::CONFIG;
        use tracing::info;

        let message = format!("{}", format!($($arg)*));
        println!("{}{message}\x1b[0m", CONFIG.message.message);
        info!("Sent message:\n\t{message}")
    }};
}

/// # Description
/// Prints to stdout with the default formatting
///
/// Unaffected by the quiet flag
#[macro_export]
macro_rules! pr {
    ($($arg:tt)*) => {{
        use twoconfig::CONFIG;
        use tracing::debug;

        let message = format!("{}", format!($($arg)*));
        println!("{}{message}\x1b[0m", CONFIG.message.default);
        debug!("Printed:\n\t{message}")
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
        use twoconfig::CONFIG;
        use tracing::trace;

        let f = std::path::Path::new(file!())
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");

        let message = format!("[{}] {}", f, format!($($arg)*));
        println!("{}{}\x1b[0m", CONFIG.message.verbose, message);
        trace!("Sent verbose message:\n\t{message}")
    }};
}
