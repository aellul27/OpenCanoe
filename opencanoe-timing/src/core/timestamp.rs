#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    /// Total time in 1/10,000 seconds (TIMY precision)
    ticks: i64,
}

impl Timestamp {
    pub fn from_hms(hours: i64, minutes: i64, seconds: i64, frac_1e4: i64) -> Self {
        let total_seconds = (hours * 3600) + (minutes * 60) + seconds;
        let ticks = (total_seconds * 10_000) + frac_1e4;
        Self { ticks }
    }

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
        // 1 second + 10000 ticks = effectively 2 seconds
        assert_eq!(ts.to_string(), "03:02:01.0000");
    }
}