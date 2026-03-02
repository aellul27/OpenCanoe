//! Core timing event types.
//!
//! This module defines the primary event types emitted and consumed by the
//! timing subsystem: an `EventId` alias, the `Channel` enum which describes
//! where on a device the event originated, and `TimingEvent` which contains
//! both system and device timestamps plus metadata.

use std::time::SystemTime;

use crate::core::Timestamp;

/// Unique identifier type for timing events.
///
/// The concrete type is `u64` today; using a type alias improves readability
/// at call sites and makes it easy to change later if needed.
pub type EventId = u64;

/// Timing channel reported by a device.
///
/// Variants correspond to common hardware channels: `Start` and `Finish`
/// represent the primary race control channels, `Intermediate(n)` covers
/// numbered intermediate channels reported by the device, and `Unknown(n)`
/// is available for vendor-specific or otherwise unrecognized channels.
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

// Example of creating a `TimingEvent`:
//
// ```rust
// use std::time::SystemTime;
// use opencanoe_timing::core::{Timestamp, TimingEvent, Channel};
//
// let event = TimingEvent {
//     id: 1,
//     system_time: SystemTime::now(),
//     device_time: Timestamp::from_hms(0, 0, 1, 2345),
//     device_name: "TST-01".to_string(),
//     device_mode: None,
//     channel: Channel::Finish,
//     raw: Some("RAWLINE".to_string()),
// };
// ```