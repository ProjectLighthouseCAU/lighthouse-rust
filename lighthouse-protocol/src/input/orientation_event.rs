use serde::{Deserialize, Serialize};

use super::EventSource;

/// A device orientation event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OrientationEvent {
    /// The client identifier.
    pub source: EventSource,
    /// Whether the device provides absolute orientation data.
    pub absolute: Option<bool>,
    /// The motion of the device around the z-axis, in degrees from 0 (inclusive) to 360 (exclusive).
    pub alpha: Option<f64>,
    /// The motion of the device around the x-axis (front to back motion), in degrees from -180 (inclusive) to 180 (exclusive).
    pub beta: Option<f64>,
    /// The motion of the device around the y-axis (left to right motion), in degrees from -90 (inclusive) to 90 (exclusive).
    pub gamma: Option<f64>,
}
