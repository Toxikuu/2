#[macro_export]
macro_rules! a {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            assert!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! d {
    ($($arg:expr),+ $(,)?) => {
        if cfg!(debug_assertions) {
            $(dbg!(&$arg);)+
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r#true() { a!(true) }

    #[test]
    #[should_panic]
    fn r#false() { a!(false) }

    #[test]
    fn check_true() {
        let x = 1;
        a!(x > 0, "x should be positive, got {}", x);
    }

    #[test]
    #[should_panic]
    fn check_false() {
        let x = -1;
        a!(x > 0, "x should be positive, got {}", x);
    }
}
