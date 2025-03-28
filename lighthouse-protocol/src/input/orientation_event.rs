use serde::{Deserialize, Serialize};

use crate::{Direction, Vec2};

use super::EventSource;

/// A device orientation event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
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

impl OrientationEvent {
    /// The approximate direction (outside of a small deadzone) for a phone tilted against a flat surface.
    pub fn direction(&self) -> Option<Direction> {
        let Some(beta) = self.beta else { return None };
        let Some(gamma) = self.gamma else { return None };

        let deadzone_radius: f64 = 10.0;
        if beta.abs().max(gamma.abs()) < deadzone_radius {
            return None;
        }

        Direction::approximate_from(Vec2::new(gamma, beta))
    }
}
