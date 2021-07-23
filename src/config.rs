use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapConfig {
    height: f32,
    width: f32,
}

impl Default for MapConfig {
    fn default() -> Self {
        return MapConfig {
            height: 10.0,
            width: 10.0,
        }
    }
}