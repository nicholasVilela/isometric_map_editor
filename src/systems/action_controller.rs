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

use crate::game::tile::Tile;

#[derive(SystemDesc, Default)]
pub struct ActionControllerSystem;

impl <'s> System<'s> for ActionControllerSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut sprite_renders, transforms, tiles, cameras, input): Self::SystemData) {
        let mouse_left_click = input.action_is_down("mouse_left_click").unwrap_or(false);

        if mouse_left_click {
            if let Some((x, y)) = input.mouse_position() {
                
            }
        }
    }
}


fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    return x >= left && x <= right && y >= bottom && y <= top
}

fn point_in_triangle(a: f32, b: f32, c: f32, p: f32) {

}