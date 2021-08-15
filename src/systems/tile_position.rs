use amethyst::{
    derive::SystemDesc,
    ecs::{WriteStorage, ReadStorage, System, SystemData, Read, Join},
    utils::{
        fps_counter::FpsCounter
    },
    ui::{UiText, UiTransform},
    renderer::{
        SpriteRender,
    },
    core::{
        transform::{Transform},
    },
    renderer::{
        Camera
    },
};

use crate::game::{
    tile::Tile,
    util::{
        map_to_screen,
        screen_to_map,
        config::GameConfig
    }
};


#[derive(SystemDesc, Default)]
pub struct TilePositionSystem;

impl <'s> System<'s> for TilePositionSystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, GameConfig>,
    );

    fn run(&mut self, (mut ui_texts, mut ui_transforms, tiles, transforms, cameras, mut sprite_renders, config): Self::SystemData) {
        for (_camera, cam_transform) in (&cameras, &transforms).join() {
            for (tile, ui_transform, ui_text, transform, sprite_render) in (&tiles, &mut ui_transforms, &mut ui_texts, &transforms, &mut sprite_renders).join() {
                ui_text.text = format!("{:?}, {:?}", tile.map_x.unwrap_or(0), tile.map_y.unwrap_or(0));

                ui_transform.local_x = ((transform.translation().x - cam_transform.translation().x) / cam_transform.scale().x) * 4.0;
                ui_transform.local_y = ((transform.translation().y - cam_transform.translation().y) / cam_transform.scale().y) * 4.0;
                
                if sprite_render.sprite_number != 0 && tile.selected {
                    sprite_render.sprite_number = 0;
                }
                else if sprite_render.sprite_number != 1 && !tile.selected {
                    sprite_render.sprite_number = 1;
                }
            }
        }
    }
}