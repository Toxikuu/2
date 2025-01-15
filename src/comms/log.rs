// src/comms/log.rs
//! Some utility functions for communicating with the user

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        println!("\x1b[{}{}\x1b[0m", CONFIG.message.message, format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! pr {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        println!("\x1b[{}{}\x1b[0m", CONFIG.message.default, format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! cpr {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        use $crate::globals::flags::FLAGS;
        if !FLAGS.lock().unwrap().quiet {
            println!("\x1b[{}{}\x1b[0m", CONFIG.message.stdout, format!($($arg)*))
        }
    }};
}

#[macro_export]
macro_rules! erm {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        eprintln!("\x1b[{}{}\x1b[0m", CONFIG.message.danger, format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! vpr {
    ($($arg:tt)*) => {{
        use $crate::globals::flags::FLAGS;
        if FLAGS.lock().unwrap().verbose {
            use $crate::globals::config::CONFIG;
            let f = std::path::Path::new(file!())
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            println!("\x1b[{}[{}] {}\x1b[0m", CONFIG.message.verbose, f, format!($($arg)*))
        }
    }};
}

pub(crate) use {msg, pr, cpr, erm, vpr};
