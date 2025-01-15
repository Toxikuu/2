// src/utils/fail.rs
//! Defines utilities for (bad) error handling (crashes)

// TODO: consider adding an erm method that discards an error and sends a message

use std::fmt;
use crate::comms::log::erm;
use crate::globals::config::CONFIG;
use std::panic::Location;

#[allow(dead_code)]
pub enum UnreachableType {
    Option,
    Result,
    Explicit,
}

pub enum FailType {
    Unreachable(UnreachableType),
    Result,
    Option,
    Explicit,
}

impl fmt::Display for FailType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Result => write!(f, "RESULT"),
            Self::Option => write!(f, "OPTION"),
            Self::Explicit => write!(f, "EXPLICIT"),
            Self::Unreachable(t) => {
                let t = match t {
                    UnreachableType::Option => "OPTION",
                    UnreachableType::Result => "RESULT",
                    UnreachableType::Explicit => "EXPLICIT",
                };
                write!(f, "UNREACHABLE {t}")
            },
        }
    }
}

macro_rules! die {
    ($($arg:tt)*) => {{
        use $crate::globals::config::CONFIG;
        println!("\x1b[{}{}\x1b[0m", CONFIG.message.danger, format!($($arg)*));

        std::panic::set_hook(Box::new(|_| {})); // suppress all panic output
        panic!();
    }};
}

pub fn report(msg: &str, location: &'static Location<'static>, fail_type: &FailType) {
    if CONFIG.general.show_bug_report_message {
        let link = match fail_type {
            FailType::Unreachable(_) => {
                "https://github.com/Toxikuu/2/issues/new?assignees=Toxikuu&labels=unreachable&projects=&template=bug.md&title=%5BBUG%5D%20%3CBrief%20Description%3E"
            },
            _ => {
                "https://github.com/Toxikuu/2/issues/new?assignees=Toxikuu&labels=bug&projects=&template=bug.md&title=%5BBUG%5D%20%3CBrief%20Description%3E"
            },
        };

        if let FailType::Unreachable(_) = fail_type {
            erm!("Please report this bug at:");
        } else {
            erm!("If you believe this to be a bug, please report it at:");
        }

        erm!("{}\n", link);
    }

    erm!("In {} on line {}, column {}", location.file(), location.line(), location.column());
    die!("[{}] {}", fail_type, msg);
}

pub trait Fail<T, E> {
    fn fail_with_location(self, msg: &str, location: &'static Location<'static>) -> T;

    #[track_caller]
    fn fail(self, msg: &str) -> T where Self: Sized {
        self.fail_with_location(msg, Location::caller())
    }

    fn ufail_with_location(self, msg: &str, location: &'static Location<'static>) -> T;

    #[track_caller]
    fn ufail(self, msg: &str) -> T where Self: Sized {
        self.ufail_with_location(msg, Location::caller())
    }
}

impl<T, E> Fail<T, E> for Result<T, E>
where 
    E: fmt::Debug,
{
    fn fail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|e| {
            let msg = &format!("{msg}: {e:?}");
            report(msg, location, &FailType::Result);
            unreachable!()
        })
    }

    fn ufail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|e| {
            let msg = &format!("{msg}: {e:?}");
            report(msg, location, &FailType::Unreachable(UnreachableType::Result));
            unreachable!()
        })
    }
}

impl<T> Fail<T, ()> for Option<T> {
    fn fail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|| {
            report(msg, location, &FailType::Option);
            unreachable!()
        })
    }

    fn ufail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|| {
            report(msg, location, &FailType::Unreachable(UnreachableType::Option));
            unreachable!()
        })
    }
}

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {{
        use $crate::utils::fail::{report, FailType};
        report(
            &format!($($arg)*),
            std::panic::Location::caller(),
            &FailType::Explicit
        );
        unreachable!()
    }};
}

#[macro_export]
macro_rules! ufail {
    ($($arg:tt)*) => {{
        use $crate::utils::fail::{report, FailType, UnreachableType};
        report(
            &format!($($arg)*),
            std::panic::Location::caller(),
            &FailType::Unreachable(UnreachableType::Explicit)
        );
        unreachable!()
    }};
}

pub(crate) use {fail, ufail};

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result, anyhow};

    #[test]
    #[allow(clippy::should_panic_without_expect)]
    #[should_panic]
    fn option_fail() {
        let option: Option<char> = None;
        option.fail("option fail test");
    }

    #[test]
    #[allow(clippy::should_panic_without_expect)]
    #[should_panic]
    fn option_ufail() {
        let option: Option<char> = None;
        option.ufail("unreachable option fail test");
    }

    #[test]
    #[allow(clippy::should_panic_without_expect)]
    #[should_panic]
    fn error_fail() {
        let result: Result<char> = Err(anyhow!("hi mom"));
        result.fail("error fail test");
    }

    #[test]
    #[allow(clippy::should_panic_without_expect)]
    #[should_panic]
    fn error_ufail() {
        let result: Result<char> = Err(anyhow!("hi mom"));
        result.ufail("unreachable error fail test");
    }

    #[test]
    fn option_success() {
        let option: Option<char> = Some('a');
        assert_eq!(option.fail("shouldn't fail"), 'a');
    }

    #[test]
    fn option_usuccess() {
        let option: Option<char> = Some('a');
        assert_eq!(option.ufail("shouldn't fail"), 'a');
    }

    #[test]
    fn error_success() {
        let result: Result<char> = Ok('a');
        assert_eq!(result.fail("shouldn't fail"), 'a');
    }

    #[test]
    fn error_usuccess() {
        let result: Result<char> = Ok('a');
        assert_eq!(result.ufail("shouldn't fail"), 'a');
    }
}
