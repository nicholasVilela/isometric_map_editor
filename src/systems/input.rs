use amethyst::{
    derive::SystemDesc,
    ecs::{
        Join,
        WriteStorage,
        ReadStorage,
        SystemData,
        System,
        Read,
        ParJoin,
    },
    core::{
        transform::{Transform},
        // timing::Time,
        math::{Point2, Vector2},
        geometry::Plane,
    },
    input::{
        InputHandler,
        StringBindings,
    },
    renderer::{
        SpriteRender,
        Camera,
    },
};


use rayon::prelude::*;

use crate::game::{
    tile::Tile,
    util::{
        map_to_screen,
        screen_to_map,
        config::GameConfig,
    }
};

#[derive(SystemDesc, Default)]
pub struct InputSystem;

impl <'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tile>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, GameConfig>,
    );

    fn run(&mut self, (mut sprite_renders, transforms, mut tiles, cameras, input, config): Self::SystemData) {
        let mouse_left_click = input.action_is_down("mouse_left_click").unwrap_or(false);

        if mouse_left_click {
            if let Some((x, y)) = input.mouse_position() {
                println!("------START------");
                for (_, cam_transform) in (&cameras, &transforms).join() {
                    let world_x = x + cam_transform.translation().x;
                    let world_y = y + cam_transform.translation().y;
                    
                    let iso_position = screen_to_map(world_x, world_y, config.tile.width, config.tile.height);

                    for tile in (&mut tiles).join() {
                        if tile.map_x.unwrap_or(0) == iso_position.x as isize && tile.map_y.unwrap_or(0) == iso_position.y as isize {
                            tile.selected = true;
                        }
                        else {
                            tile.selected = false;
                        }
                    }

                    println!("Mouse Position: ({:?}, {:?})", world_x, world_y);
                    println!("Grid Position: ({:?}, {:?})", iso_position.x, iso_position.y);
                }
            }
        }
    }
}


fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    return x >= left && x <= right && y >= bottom && y <= top
}