use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
pub struct LaserMetrics {
    pub rooms: Vec<RoomMetrics>,
}

#[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
pub struct RoomMetrics {
    pub room: String,
    #[serde(flatten)]
    pub api: RoomApiMetrics,
}

// FIXME: Once https://github.com/serde-rs/serde/pull/2525 and
// https://github.com/serde-rs/serde/issues/745 are merged/fixed, we can
// hopefully do it like this:

// #[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
// #[serde(tag = "api_version")]
// pub enum RoomApiMetrics {
//     #[serde(rename = 1)]
//     V1(RoomV1Metrics),
//     #[serde(rename = 2)]
//     V2(RoomV2Metrics),
// }

// The untagged struct does not serialize correctly (it omits the api_version),
// therefore we intentionally leave the Serialize conformance out for now.

#[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum RoomApiMetrics {
    V1(RoomV1Metrics),
    V2(RoomV2Metrics),
}

#[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
pub struct RoomV1Metrics {
    pub controller_metrics: BoardV1Metrics,
    pub lamp_metrics: HashMap<i32, BoardV1Metrics>,
}

#[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
pub struct BoardV1Metrics {
    pub id: i32,
    pub version: i32,
    pub uptime: i32,
    pub temperature: i32,
    pub init_temperature: i32,
    pub settings: String,
    pub timeout: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_responding: Option<bool>,
}

#[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
pub struct RoomV2Metrics {
    pub controller_metrics: ControllerV2Metrics,
    pub lamp_metrics: Vec<LampV2Metrics>,
}

#[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
pub struct ControllerV2Metrics {
    pub responding: bool,
    pub ping_latency_ms: f64,
    pub firmware_version: i32,
    pub uptime: i32,
    pub frames: i32,
    pub fps: i32,
    pub core_temperature: f64,
    pub board_temperature: f64,
    pub shunt_voltage: f64,
    pub voltage: f64,
    pub power: f64,
    pub current: f64,
}

#[derive(Debug, /* Serialize,*/ Deserialize, PartialEq, Clone)]
pub struct LampV2Metrics {
    pub responding: bool,
    pub firmware_version: i32,
    pub uptime: i32,
    pub timeout: i32,
    pub temperature: i32,
    pub fuse_tripped: bool,
    pub flashing_status: String,
}
