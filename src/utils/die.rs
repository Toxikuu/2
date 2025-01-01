// src/traits.rs

use std::fmt::Display;
use crate::die;

pub trait Fail<T, E> {
    fn fail(self, msg: &str) -> T;
}

impl<T, E> Fail<T, E> for Result<T, E>
where 
    E: Display,
{
    fn fail(self, msg: &str) -> T {
        self.unwrap_or_else(|_| die!("{}", msg))
    }
}

impl<T> Fail<T, ()> for Option<T> {
    fn fail(self, msg: &str) -> T {
        self.unwrap_or_else(|| die!("{}", msg))
    }
}
