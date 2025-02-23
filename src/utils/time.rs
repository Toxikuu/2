// src/utils/time.rs
//! Provides utility functions for dealing with time

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use super::fail::Fail;

pub fn timestamp() -> Duration {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).fail("Back to the future")
}

/// # Description
/// Stopwatch is a helper struct for timing sections of code
/// It's mostly used for PM endpoints
pub struct Stopwatch {
    pub start_time: Option<Instant>,
    elapsed: Duration,
}

impl Stopwatch {
    pub const fn new() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::ZERO,
        }
    }

    /// # Description
    /// Starts (or resumes) the stopwatch
    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    /// # Description
    /// Stops (pauses) the stopwatch
    pub fn stop(&mut self) {
        if let Some(start_time) = self.start_time {
            self.elapsed += start_time.elapsed();
            self.start_time = None;
        }
    }

    // /// # Description
    // /// Resets the stopwatch
    // /// UNUSED
    // #[allow(dead_code)]
    // pub fn reset(&mut self) {
    //     self.start_time = None;
    //     self.elapsed = Duration::ZERO;
    // }

    /// # Description
    /// Returns the total elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.map_or(
            self.elapsed, |start_time| self.elapsed + start_time.elapsed()
        )
    }

    /// # Description
    /// Displays the elapsed time in a human-readable format
    pub fn display(&self) -> String {
        self.elapsed().pretty()
    }
}

/// # Description
/// Adds the ``pretty()`` method for Duration
/// This is used by ``Stopwatch::display()``
trait Pretty{
    fn pretty(self) -> String;
}

impl Pretty for Duration {
    /// # Description
    /// Displays duration in a human-readable, dare I say pretty, format
    fn pretty(self) -> String {
        let total_millis = self.as_millis_f32();
        if total_millis < 1. {
            format!("{} ns", self.as_nanos())
        } else if total_millis < 1_000. {
            format!("{total_millis:.3} ms")
        } else if total_millis < 60_000. {
            format!("{:.3} s", self.as_secs_f32())
        } else {
            let total_secs = self.as_secs();
            if total_secs < 3_600 {
                format!("{}:{:02} min", total_secs / 60, total_secs % 60)
            } else {
                format!(
                    "{}:{:02}:{:02} hrs",
                    total_secs / 3_600,
                    total_secs % 3_600 / 60,
                    total_secs % 60,
                )
            }
        }
    }
}
