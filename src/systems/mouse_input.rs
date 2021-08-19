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
        timing::Time,
        math::Vector3,
    },
    input::{
        InputHandler,
        StringBindings,
    },
    renderer::{
        Camera,
    },
};

use rayon::prelude::*;

use crate::game::{
    tile::Tile,
    util::{
        screen_to_map,
        config::GameConfig,
    }
};

#[derive(SystemDesc)]
pub struct MouseInputSystem {
    pub move_speed: f32,
    pub zoom_step: f32,
    pub zoom_min: f32,
    pub zoom_max: f32,
}

impl Default for MouseInputSystem {
    fn default() -> Self {
        return MouseInputSystem {
            move_speed: 30.0,
            zoom_step: 0.1,
            zoom_min: 0.8,
            zoom_max: 30.0,
        };
    }
}

impl MouseInputSystem {
    fn camera_movement(&self, cam_transform: &mut Transform, input: &Read<InputHandler<StringBindings>>, time: &Read<Time>) {
        let d_x = input.axis_value("mouse_x").unwrap_or(0.0);
        let d_y = input.axis_value("mouse_y").unwrap_or(0.0);

        let x =  d_x * self.move_speed * cam_transform.scale().x * time.delta_seconds();
        let y = -d_y * self.move_speed * cam_transform.scale().x * time.delta_seconds();

        cam_transform.move_right(x);
        cam_transform.move_up(y);
    }

    fn camera_zoom(&self, cam_transform: &mut Transform, mouse_wheel: f32) {
        let scale = cam_transform.scale();
        let zoom_in = mouse_wheel > 0.0;

        if zoom_in && scale.x > self.zoom_min {
            cam_transform.set_scale(Vector3::new(scale.x - self.zoom_step, scale.y - self.zoom_step, scale.z));
        }
        else if !zoom_in && scale.x < self.zoom_max {
            cam_transform.set_scale(Vector3::new(scale.x + self.zoom_step, scale.y + self.zoom_step, scale.z));
        }
    }

    fn tile_select(&self, cam_transform: &Transform, config: &Read<GameConfig>, tiles: &mut WriteStorage<Tile>, input: &Read<InputHandler<StringBindings>>) {
        if let Some((x, y)) = input.mouse_position() {
            let world_x = x - ((200.0) - (cam_transform.translation().x * 4.0));
            let world_y = y - ((200.0) + (cam_transform.translation().y * 4.0) - ((config.tile.height) * 3.0));
            
            let map_position = screen_to_map(world_x, world_y, config.tile.width, config.tile.height);

            (tiles).par_join().for_each(|tile| {
                if !tile.selected && tile.map_x.unwrap_or(0) == map_position.x as isize && tile.map_y.unwrap_or(0) == map_position.y as isize {
                    tile.selected = true;
                }
                else if tile.selected && !(tile.map_x.unwrap_or(0) == map_position.x as isize && tile.map_y.unwrap_or(0) == map_position.y as isize){
                    tile.selected = false;
                }
            });
        }
    }
}

impl <'s> System<'s> for MouseInputSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tile>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, GameConfig>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut tiles, cameras, input, config, time): Self::SystemData) {
        let left_click = input.action_is_down("mouse_left_click").unwrap_or(false);
        let middle_down = input.action_is_down("mouse_middle").unwrap_or(false);
        let mouse_wheel = input.axis_value("mouse_wheel").unwrap_or(0.0);

        if left_click || middle_down || mouse_wheel != 0.0 {
            for (_cam, cam_transform) in (&cameras, &mut transforms).join() {
                if left_click {self.tile_select(cam_transform, &config, &mut tiles, &input);}
                if middle_down {self.camera_movement(cam_transform, &input, &time);}
                if mouse_wheel != 0.0 {self.camera_zoom(cam_transform, mouse_wheel);}
            }
        }
    }
}