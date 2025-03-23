use serde::{Deserialize, Serialize};

use crate::{Rot3, Vec3};

use super::EventSource;

/// A device motion event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MotionEvent {
    /// The client identifier.
    pub source: EventSource,
    /// The acceleration in 3D space in m/s^2.
    pub acceleration: Option<Vec3<Option<f64>>>,
    /// The acceleration in 3D space (including gravity) in m/s^2.
    pub acceleration_including_gravity: Option<Vec3<Option<f64>>>,
    /// The rotation rate in deg/s on the three rotation axes.
    pub rotation_rate: Option<Rot3<Option<f64>>>,
    /// The granularity of these events in ms.
    pub interval: f64,
}
