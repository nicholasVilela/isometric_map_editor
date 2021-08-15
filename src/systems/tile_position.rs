use amethyst::{
    derive::SystemDesc,
    ecs::{WriteStorage, ReadStorage, System, SystemData, Read, Join},
    utils::{
        fps_counter::FpsCounter
    },
    ui::{UiText, UiTransform},
    core::{
        transform::{Transform},
    },
    renderer::{
        Camera
    },
};

use rayon::prelude::*;

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
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (tiles, transforms, mut ui_texts, mut ui_transforms, cameras): Self::SystemData) {
        for (_camera, cam_transform) in (&cameras, &transforms).join() {
            for (tile, transform, ui_transform, ui_text) in (&tiles, &transforms, &mut ui_transforms, &mut ui_texts).join() {
                if tile.selected {
                    ui_text.text = format!("{:?}, {:?}", tile.map_x.unwrap_or(0), tile.map_y.unwrap_or(0));

                    ui_transform.local_x = ((transform.translation().x - cam_transform.translation().x) / cam_transform.scale().x) * 4.0;
                    ui_transform.local_y = ((transform.translation().y - cam_transform.translation().y) / cam_transform.scale().y) * 4.0;
                }

            }
        }
    }
}