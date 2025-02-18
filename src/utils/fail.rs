// src/utils/fail.rs
//! Defines utilities for (bad) error handling (crashes)

#[cfg(not(test))]
use crate::{
    comms::out::erm,
    utils::logger,
    globals::config::CONFIG,
};
use std::{
    fmt,
    panic::Location,
    panic,
};

/// # Description
/// A utility macro which panics with custom formatting, suppressing the default panic output
#[cfg(not(test))]
macro_rules! die {
    ($($arg:tt)*) => {{
        erm!("{}", format!($($arg)*));
        std::panic::set_hook(Box::new(|_| {})); // suppress all panic output
        panic!();
    }};
}

/// # Description
/// Reports the cause of a failure before panicing
///
/// If the failure should be unreachable, prompts the user to report it as a bug featuring a github
/// issue link
#[cold]
#[track_caller]
#[cfg(not(test))]
pub fn report(msg: &str, location: &'static Location<'static>) -> ! {
    if CONFIG.general.show_bug_report_message {
        const LINK: &str = "https://github.com/Toxikuu/2/issues/new?assignees=Toxikuu&labels=bug&projects=&template=bug.md&title=%5BBUG%5D%20%3CBrief%20Description%3E";
        erm!("If you believe this to be a bug, please report it at:");
        erm!("{LINK}\n");
    }

    let loc_msg = format!("Failure in {} @ {}:{}", location.file(), location.line(), location.column());
    logger::get().detach();
    log::debug!("{loc_msg}");
    log::debug!("{msg}");
    log::error!("Process died\n\n\t----------------\n");
    if CONFIG.general.show_failure_location {
        erm!("{loc_msg}");
    }
    die!("{msg}");
}

#[cfg(test)]
#[allow(clippy::panic)]
pub fn report(msg: &str, _location: &'static Location<'static>) -> ! {
    panic!("{msg}")
}

/// # Description
/// The Fail trait allows you to call ``.fail()`` and ``.fail()`` on result and option types
///
/// These then call report, which "gracefully" panics
///
/// ``fail_with_location()`` and ``fail_with_location()`` should not be used
///
/// **Examples:**
/// ```rust
/// fn fallible_function() -> anyhow::Result<()> {
///     bail!("hi mom");
///     Ok(())
/// }
///
/// // ``.fail()`` will also output the error message
/// fallible_function().fail("Fallible function failed");
///
/// let num: Option<u8> = None;
/// num.fail("Num was none");
///
/// let num: Option<u8> = Some(42);
/// num.fail("Shouldn't have failed");
///
/// println!("Number: {}", num); // should output ``Number: 42``
///
/// ```
pub trait Fail<T, E> {
    fn fail(self, msg: &str) -> T;
}

impl<T, E> Fail<T, E> for Result<T, E>
where
    E: fmt::Debug,
{
    #[track_caller]
    fn fail(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                let err = format!("{e:#?}").lines().map(|l| format!("\t{l}")).collect::<Vec<_>>().join("\n");
                let msg = &format!("{msg}:\n{err}");
                let location = Location::caller();
                report(msg, location);
            }
        }
    }
}

impl<T> Fail<T, ()> for Option<T> {
    #[track_caller]
    fn fail(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                let location = Location::caller();
                report(msg, location);
            } 
        }
    }
}

pub trait BoolFail {
    fn and_fail(self, msg: &str);
    fn or_fail(self, msg: &str);
}

impl BoolFail for bool {
    #[track_caller]
    fn and_fail(self, msg: &str) {
        if self {
            let location = Location::caller();
            report(msg, location);
        }
    }

    #[track_caller]
    fn or_fail(self, msg: &str) {
        if !self {
            let location = Location::caller();
            report(msg, location);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn option_success() {
        let option: Option<char> = Some('a');
        assert_eq!(option.fail("shouldn't fail because option was some"), 'a');
    }

    #[allow(clippy::should_panic_without_expect)]
    #[should_panic]
    #[test]
    fn option_fail() {
        let option: Option<char> = None;
        option.fail("should fail because option was none");
    }

    #[test]
    fn error_success() {
        let result: Result<char> = Ok('a');
        assert_eq!(result.fail("shouldn't fail because result was ok"), 'a');
    }

    #[allow(clippy::should_panic_without_expect)]
    #[should_panic]
    #[test]
    fn error_fail() {
        let result = std::fs::read_to_string("/usr");
        result.fail("should fail because /usr is a dir");
    }

    #[test]
    fn bool_success() {
        let condition = true;
        condition.or_fail("shouldn't fail because condition was met");
    }

    #[allow(clippy::should_panic_without_expect)]
    #[should_panic]
    #[test]
    fn bool_fail() {
        let condition = true;
        condition.and_fail("should fail because condition was met");
    }
}
