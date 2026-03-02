//! Timestamp utilities for TIMY-style timing.
//!
//! This module provides `Timestamp`, a compact representation of a time value
//! stored as an integer count of 1/10,000 seconds (TIMY precision). It offers
//! a simple constructor and a human-readable formatter used in tests and
//! debugging.

/// A timestamp measured in "ticks" where one tick = 1/10,000 second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    /// Total time in 1/10,000 seconds (TIMY precision).
    ///
    /// This value is stored as an `i64` count of 1/10,000-second intervals
    /// since zero. The field is private; create values via `from_hms` and
    /// format them with `to_string`.
    ticks: i64,
}

impl Timestamp {
    /// Construct a `Timestamp` from hours, minutes, seconds and a fractional
    /// part expressed in 1/10,000ths of a second (`frac_1e4`).
    ///
    /// The function accepts values that may be outside usual ranges (for
    /// example `minutes = 120` or `frac_1e4 = 10_000`). All inputs are
    /// converted into the internal tick count and will carry into higher
    /// units when necessary (i.e. fractional ticks >= 10_000 become whole
    /// seconds, extra minutes become hours, etc.). This mirrors the current
    /// implementation which does arithmetic directly on totals.
    ///
    /// Example:
    ///
    /// ```rust
    /// use opencanoe_timing::core::timestamp::Timestamp;
    /// let ts = Timestamp::from_hms(1, 2, 3, 4567);
    /// assert_eq!(ts.to_string(), "01:02:03.4567");
    /// ```
    pub fn from_hms(hours: i64, minutes: i64, seconds: i64, frac_1e4: i64) -> Self {
        let total_seconds = (hours * 3600) + (minutes * 60) + seconds;
        let ticks = (total_seconds * 10_000) + frac_1e4;
        Self { ticks }
    }

    /// Format the `Timestamp` as `HH:MM:SS.mmmm` where `mmmm` is the 4-digit
    /// fractional part in 1/10,000 seconds. All fields are zero-padded to the
    /// widths shown.
    ///
    /// Formatting derives hours/minutes/seconds from the total tick count and
    /// displays the fractional remainder with exactly four digits. The method
    /// assumes non-negative timestamps for human-friendly output; negative
    /// values are not specially handled by the current implementation.
    pub fn to_string(&self) -> String {
        let total_seconds = self.ticks / 10_000;
        let frac = self.ticks % 10_000;

        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        format!("{:02}:{:02}:{:02}.{:04}", hours, minutes, seconds, frac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hms_basic() {
        let ts = Timestamp::from_hms(1, 2, 3, 4567);
        assert_eq!(ts.ticks, (1 * 3600 + 2 * 60 + 3) * 10_000 + 4567);
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
}