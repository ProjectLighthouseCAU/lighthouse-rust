use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LaserMetrics {
    pub rooms: Vec<RoomMetrics>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RoomMetrics {
    pub room: String,
    pub api_version: i32,
    pub controller_metrics: HashMap<String, Value>,
    pub lamp_metrics: Value,
}
