use amethyst::{
    prelude::*,
    core::{
        math::{Vector2}
    }
};

#[path = "./config.rs"]
pub mod config;
pub use config::GameConfig;


// pub fn map_to_screen(x: i32, y: i32, tile_width: f32, tile_height: f32) -> Vector2<f32> {
//     let offset_x = 0.0;
//     let offset_y = 0.0;

//     let screen_x = offset_x + (x - y) as f32 * (tile_width / 2.0);
//     let screen_y = offset_y + (x + y) as f32 * (tile_height / -2.0);

//     return Vector2::new(screen_x, screen_y);
// }

pub fn map_to_screen(x: i32, y: i32, z: i32, tile_width: f32, tile_height: f32) -> Vector2<f32> {
    let offset_x = 0.0;
    let offset_y = z as f32 * tile_height;

    let screen_x = offset_x + (x - y) as f32 * (tile_width / 2.0);
    let screen_y = offset_y + (x + y) as f32 * (tile_height / -2.0);

    return Vector2::new(screen_x, screen_y);
}

pub fn screen_to_map(x: f32, y: f32, tile_width: f32, tile_height: f32) -> Vector2<i32> {
    let offset_x = x;
    let offset_y = y;
    
    let tile_width_half = tile_width / 2.0;
    let tile_height_half = tile_height / 2.0;

    let map_x = (offset_y / tile_height_half + offset_x / tile_width_half) / 8.0;
    let map_y = (offset_y / tile_height_half - offset_x / tile_width_half) / 8.0;

    return Vector2::new(map_x.floor() as i32, map_y.floor() as i32);
}


pub fn get_map_dimensions(world: &mut World) -> (f32, f32) {
    let config = world.read_resource::<GameConfig>();

    return (config.map.height, config.map.width);
}

pub fn get_tile_dimensions(world: &mut World) -> (f32, f32) {
    let config = world.read_resource::<GameConfig>();

    return (config.tile.height, config.tile.width);
}

pub fn get_window_dimensions(world: &mut World) -> (f32, f32) {
    let config = world.read_resource::<GameConfig>();

    return (config.window.height, config.window.width);
}