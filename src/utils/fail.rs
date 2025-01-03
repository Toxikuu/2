// src/utils/fail.rs
//! Defines utilities for (bad) error handling (crashes)

use std::fmt::{self, Display};
use crate::{die, erm};
use std::panic::Location;

pub enum UnreachableType {
    Option,
    Result,
}

pub enum FailType {
    Unreachable(UnreachableType),
    Result,
    Option,
    Custom(String),
}

impl fmt::Display for FailType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FailType::Result => write!(f, "RESULT"),
            FailType::Option => write!(f, "OPTION"),
            FailType::Custom(t) => write!(f, "{}", t),
            FailType::Unreachable(t) => {
                let t = match t {
                    UnreachableType::Option => "OPTION",
                    UnreachableType::Result => "RESULT",
                };
                write!(f, "UNREACHABLE {}", t)
            },
        }
    }
}

pub fn report(msg: &str, location: &'static Location<'static>, fail_type: FailType) {
    let link = match fail_type {
        FailType::Unreachable(_) => {
            "https://github.com/Toxikuu/2/issues/new?assignees=Toxikuu&labels=unreachable&projects=&template=bug.md&title=%5BBUG%5D%20%3CBrief%20Description%3E"
        },
        _ => {
            "https://github.com/Toxikuu/2/issues/new?assignees=Toxikuu&labels=bug&projects=&template=bug.md&title=%5BBUG%5D%20%3CBrief%20Description%3E"
        },
    };

    match fail_type {
        FailType::Unreachable(_) => erm!("Please report this bug at:"),
        _ => erm!("If you believe this to be a bug, please report it at:")
    }

    erm!("{}\n", link);
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
    E: Display,
{
    fn fail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|e| {
            let msg = &format!("{}: {}", msg, e);
            report(msg, location, FailType::Result);
            unreachable!()
        })
    }

    fn ufail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|e| {
            let msg = &format!("{}: {}", msg, e);
            report(msg, location, FailType::Unreachable(UnreachableType::Result));
            unreachable!()
        })
    }
}

impl<T> Fail<T, ()> for Option<T> {
    fn fail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|| {
            report(msg, location, FailType::Option);
            unreachable!()
        })
    }

    fn ufail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|| {
            report(msg, location, FailType::Unreachable(UnreachableType::Option));
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
            FailType::Custom("MACRO".to_string())
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fmt;

    /// simple error type for testing
    #[derive(Debug)]
    struct TestError;

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "test error")
        }
    }

    impl Error for TestError {}


    #[test]
    #[should_panic]
    fn option_fail() {
        let option: Option<char> = None;
        option.fail("option fail test");
    }

    #[test]
    #[should_panic]
    fn option_ufail() {
        let option: Option<char> = None;
        option.ufail("unreachable option fail test");
    }

    #[test]
    #[should_panic]
    fn error_fail() {
        let result: Result<char, Box<dyn Error>> = Err(Box::new(TestError));
        result.fail("error fail test");
    }

    #[test]
    #[should_panic]
    fn error_ufail() {
        let result: Result<char, Box<dyn Error>> = Err(Box::new(TestError));
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
        let result: Result<char, Box<dyn Error>> = Ok('a');
        assert_eq!(result.fail("shouldn't fail"), 'a');
    }

    #[test]
    fn error_usuccess() {
        let result: Result<char, Box<dyn Error>> = Ok('a');
        assert_eq!(result.ufail("shouldn't fail"), 'a');
    }
}
