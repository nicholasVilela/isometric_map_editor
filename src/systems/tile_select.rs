use amethyst::{
    derive::SystemDesc,
    ecs::{ParJoin, WriteStorage, ReadStorage, System, SystemData, Read, Join},
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
pub struct TileSelectSystem;

impl <'s> System<'s> for TileSelectSystem {
    type SystemData = (
        ReadStorage<'s, Tile>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (tiles, mut sprite_renders): Self::SystemData) {
        (&tiles, &mut sprite_renders).par_join().for_each(|(tile, sprite_render)| {
            if sprite_render.sprite_number != 0 && tile.selected {
                sprite_render.sprite_number = 0;
            }
            else if sprite_render.sprite_number != 1 && !tile.selected {
                sprite_render.sprite_number = 1;
            }
        });
    }
}