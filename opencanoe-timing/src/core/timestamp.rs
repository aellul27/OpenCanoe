//! Timestamp utilities for TIMY-style timing.
//!
//! This module provides `Timestamp`, a compact representation of a time value
//! stored as an integer count of 1/10,000 seconds (TIMY precision). It offers
//! a simple constructor and a human-readable formatter used in tests and
//! debugging.

use std::fmt;

/// A timestamp measured in "ticks" where one tick = 1/10,000 second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    /// Total time in 1/10,000 seconds (TIMY precision).
    ///
    /// This value is stored as an `i64` count of 1/10,000-second intervals
    /// since zero. The field is private; create values via `from_hms` and
    /// format them with `Display`.
    ticks: i64,
}

impl Timestamp {
    /// Construct a `Timestamp` from hours, minutes, seconds and a fractional
    /// part expressed in 1/10,000ths of a second (`frac_1e4`).
    ///
    /// The function accepts values that may be outside usual ranges (for
    /// example `minutes = 120` or `frac_1e4 = 10_000`). All inputs are
    /// converted into the internal tick count and will carry into higher
    /// units when necessary (i.e. fractional ticks >= `10_000` become whole
    /// seconds, extra minutes become hours, etc.). This mirrors the current
    /// implementation which does arithmetic directly on totals.
    ///
    /// # Panics
    ///
    /// Panics if any input is negative. Negative timestamps are not supported.
    ///
    /// Example:
    ///
    /// ```rust
    /// use opencanoe_timing::core::timestamp::Timestamp;
    /// let ts = Timestamp::from_hms(1, 2, 3, 4567);
    /// assert_eq!(ts.to_string(), "01:02:03.4567");
    /// ```
    #[must_use]
    pub fn from_hms(hours: i64, minutes: i64, seconds: i64, frac_1e4: i64) -> Self {
        assert!(hours >= 0, "hours must be non-negative");
        assert!(minutes >= 0, "minutes must be non-negative");
        assert!(seconds >= 0, "seconds must be non-negative");
        assert!(frac_1e4 >= 0, "fraction must be non-negative");

        let total_seconds = (hours * 3600) + (minutes * 60) + seconds;
        let ticks = (total_seconds * 10_000) + frac_1e4;
        Self { ticks }
    }
}

impl fmt::Display for Timestamp {
    /// Format the `Timestamp` as `HH:MM:SS.mmmm` where `mmmm` is the 4-digit
    /// fractional part in 1/10,000 seconds. All fields are zero-padded to the
    /// widths shown.
    ///
    /// Formatting derives hours/minutes/seconds from the total tick count and
    /// displays the fractional remainder with exactly four digits.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_seconds = self.ticks / 10_000;
        let frac = self.ticks % 10_000;

        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        write!(f, "{hours:02}:{minutes:02}:{seconds:02}.{frac:04}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hms_basic() {
        let ts = Timestamp::from_hms(1, 2, 3, 4567);
        assert_eq!(ts.ticks, (3600 + 2 * 60 + 3) * 10_000 + 4567);
    }

    #[test]
    fn test_to_string_basic() {
        let ts = Timestamp::from_hms(1, 2, 3, 4567);
        assert_eq!(ts.to_string(), "01:02:03.4567");
    }

    #[test]
    fn test_zero_time() {
        let ts = Timestamp::from_hms(0, 0, 0, 0);
        assert_eq!(ts.ticks, 0);
        assert_eq!(ts.to_string(), "00:00:00.0000");
    }

    #[test]
    fn test_max_fraction_padding() {
        let ts = Timestamp::from_hms(0, 0, 1, 5);
        assert_eq!(ts.to_string(), "00:00:01.0005");
    }

    #[test]
    fn test_large_time() {
        let ts = Timestamp::from_hms(12, 34, 56, 7890);
        assert_eq!(ts.to_string(), "12:34:56.7890");
    }

    #[test]
    fn test_rollover_seconds() {
        let ts = Timestamp::from_hms(0, 59, 59, 9999);
        assert_eq!(ts.to_string(), "00:59:59.9999");
    }

    #[test]
    fn test_ordering() {
        let a = Timestamp::from_hms(0, 0, 1, 0);
        let b = Timestamp::from_hms(0, 0, 2, 0);
        assert!(a < b);
    }

    #[test]
    fn test_equality() {
        let a = Timestamp::from_hms(1, 1, 1, 1);
        let b = Timestamp::from_hms(1, 1, 1, 1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_overflow() {
        let ts = Timestamp::from_hms(1, 120, 120, 10_000);
        assert_eq!(ts.to_string(), "03:02:01.0000");
    }

    #[test]
    #[should_panic(expected = "hours must be non-negative")]
    fn test_negative_hours_panics() {
        let _ = Timestamp::from_hms(-1, 0, 0, 0);
    }

    #[test]
    #[should_panic(expected = "fraction must be non-negative")]
    fn test_negative_fraction_panics() {
        let _ = Timestamp::from_hms(0, 0, 0, -1);
    }
}