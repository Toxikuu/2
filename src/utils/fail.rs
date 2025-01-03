// src/utils/fail.rs

use std::fmt::Display;
use crate::{die, erm};
use std::panic::Location;

pub trait Fail<T, E> {
    fn fail(self, msg: &str) -> T;

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
    fn fail(self, msg: &str) -> T {
        self.unwrap_or_else(|_| die!("{}", msg))
    }

    fn ufail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|e| {
            let escaped_message = msg.replace(' ', "%20");
            erm!("You've managed to reach an unreachable error. Good job! Please report this at:");
            erm!("https://github.com/Toxikuu/2/issues/new?assignees=Toxikuu&labels=unreachable&projects=&template=unreachable-error.md&title=%5BUNREACHABLE%5D+Reached+%27{}%27\n", escaped_message);
            erm!("[UNREACHABLE] {}: {}", msg, e);
            erm!("[UNREACHABLE] In {} on line {}, column {}", location.file(), location.line(), location.column());
            die!("Unreachable error reached!")
        })
    }
}

impl<T> Fail<T, ()> for Option<T> {
    fn fail(self, msg: &str) -> T {
        self.unwrap_or_else(|| die!("{}", msg))
    }

    fn ufail_with_location(self, msg: &str, location: &'static Location<'static>) -> T {
        self.unwrap_or_else(|| {
            let escaped_message = msg.replace(' ', "%20");

            erm!("You've managed to reach an unreachable option. Good job! Please report this at:");
            erm!("https://github.com/Toxikuu/2/issues/new?assignees=Toxikuu&labels=unreachable&projects=&template=unreachable-error.md&title=%5BUNREACHABLE%5D+Reached+%27{}%27\n", escaped_message);

            erm!("[UNREACHABLE] {}", msg);
            erm!("[UNREACHABLE] In {} on line {}, column {}", location.file(), location.line(), location.column());
            die!("Unreachable option reached!")
        })
    }
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
    #[should_panic(expected = "option fail test")]
    fn option_fail() {
        let option: Option<char> = None;
        option.fail("option fail test");
    }

    #[test]
    #[should_panic] // expected not specified bc ufail outputs a lot
    fn option_ufail() {
        let option: Option<char> = None;
        option.ufail("unreachable option fail test");
    }

    #[test]
    #[should_panic(expected = "error fail test")]
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
