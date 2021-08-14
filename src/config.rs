use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for MapConfig {
    fn default() -> Self {
        return MapConfig {
            height: 10.0,
            width: 10.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TileConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for TileConfig {
    fn default() -> Self {
        return TileConfig {
            height: 32.0,
            width: 32.0,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        return WindowConfig {
            height: 100.0,
            width: 100.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub map: MapConfig,
    pub tile: TileConfig,
    pub window: WindowConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        return GameConfig {
            map: MapConfig::default(),
            tile: TileConfig::default(),
            window: WindowConfig::default(),
        }
    }
}