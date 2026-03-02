//! Core timing event types.
//!
//! This module defines the primary event types emitted and consumed by the
//! timing subsystem: an `EventId` alias, the `Channel` enum which describes
//! where on a device the event originated, and `TimingEvent` which contains
//! both system and device timestamps plus metadata.

use std::time::SystemTime;

use crate::core::Timestamp;

/// Unique identifier for a timing event.
pub type EventId = u64;

/// Timing channel reported by a device.
///
/// Variants correspond to common hardware channels: `Start` and `Finish`
/// represent the primary race control channels, `Intermediate(n)` covers
/// numbered intermediate channels reported by the device, and `Unknown(n)`
/// is available for vendor-specific or otherwise unrecognized channels.
///
/// # Examples
///
/// ```rust
/// use opencanoe_timing::core::Channel;
///
/// let s = Channel::Start;
/// let f = Channel::Finish;
/// let i = Channel::Intermediate(2);
/// let u = Channel::Unknown(9);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// Start channel (commonly c0).
    Start,

    /// Finish channel (commonly c1).
    Finish,

    /// Intermediate channel with index (e.g. c2..cN).
    Intermediate(u8),

    /// A channel reported by a device that doesn't match known semantics.
    Unknown(u8),
}

/// Core timing event structure containing system and device-level timestamps
/// together with metadata from the device that produced the event.
///
/// # Example
///
/// ```rust
/// use std::time::SystemTime;
/// use opencanoe_timing::core::{Timestamp, TimingEvent, Channel};
///
/// let event = TimingEvent {
///     id: 1,
///     system_time: SystemTime::now(),
///     device_time: Timestamp::from_hms(0, 0, 1, 2345),
///     device_name: "TST-01".to_string(),
///     device_mode: None,
///     channel: Channel::Finish,
///     raw: Some("RAWLINE".to_string()),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct TimingEvent {
    /// Event identifier used to correlate or reference this event.
    pub id: EventId,

    /// System-local timestamp captured when the event was observed. This is
    /// useful for logging and ordering events from the host's perspective.
    pub system_time: SystemTime,

    /// Device-local timestamp reported by the timing hardware.
    pub device_time: Timestamp,

    /// Human-readable device name (e.g. serial number, model, or user label).
    pub device_name: String,

    /// Optional device mode or configuration string (if available).
    pub device_mode: Option<String>,

    /// Channel on which the event was recorded.
    pub channel: Channel,

    /// Raw payload or line received from the device (unparsed), if present.
    pub raw: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use crate::core::Timestamp;

    #[test]
    fn channel_equality_and_copy() {
        let a = Channel::Start;
        let b = a; // copy
        assert_eq!(a, b);

        let i1 = Channel::Intermediate(2);
        let i2 = i1;
        assert_eq!(i1, i2);
    }

    #[test]
    fn channel_matching_values() {
        let ci = Channel::Intermediate(5);
        match ci {
            Channel::Intermediate(n) => assert_eq!(n, 5),
            _ => panic!("expected Intermediate"),
        }

        let cu = Channel::Unknown(9);
        match cu {
            Channel::Unknown(n) => assert_eq!(n, 9),
            _ => panic!("expected Unknown"),
        }
    }

    #[test]
    fn timing_event_construction_and_clone() {
        let ts = Timestamp::from_hms(0, 0, 1, 2345);
        let ev = TimingEvent {
            id: 42,
            system_time: SystemTime::now(),
            device_time: ts,
            device_name: "TST-01".to_string(),
            device_mode: Some("MODE-A".to_string()),
            channel: Channel::Finish,
            raw: Some("RAWLINE".to_string()),
        };

        // basic field checks
        assert_eq!(ev.id, 42);
        assert_eq!(ev.device_time, ts);
        assert_eq!(ev.device_name, "TST-01");
        assert_eq!(ev.device_mode.as_deref(), Some("MODE-A"));
        assert_eq!(ev.channel, Channel::Finish);
        assert_eq!(ev.raw.as_deref(), Some("RAWLINE"));

        // Clone should preserve data
        let ev2 = ev.clone();
        assert_eq!(ev2.id, ev.id);
        assert_eq!(ev2.device_time, ev.device_time);
        assert_eq!(ev2.device_name, ev.device_name);
    }
}