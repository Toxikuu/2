// src/macros.rs

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

#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        println!("\x1b[{}{}\x1b[0m", CONFIG.message.danger, format!($($arg)*));

        std::panic::set_hook(Box::new(|_| {})); // suppress all panic output
        panic!();
    }};
}

#[macro_export]
macro_rules! select {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        use std::io::{self, Write};
        let mut input = String::new();
        
        print!("\x1b[{}{}: \x1b[0m", CONFIG.message.prompt, format!($($arg)*));
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_string();
        input
    }};
}

#[macro_export]
macro_rules! pkgexec {
    ($cmd:expr, $pkg:expr) => {{
        use $crate::shell::cmd::exec;
        let relpath = &$pkg.relpath;
        let command = format!(
        r#"
        PORT="/usr/ports/{}"
        SRC="$PORT/.sources"
        BLD="$PORT/.build"

        {}
        "#,
        relpath,
        $cmd,
        );

        exec(&command)
    }};
}
