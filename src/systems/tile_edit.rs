use amethyst::{
    ecs::{WriteStorage, ReadStorage, System, SystemData, Join, Read, Entities, LazyUpdate, ParJoin},
    renderer::{SpriteRender,},
    derive::{SystemDesc,},
    input::{
        InputHandler,
        StringBindings,
    },
    core::{
        transform::{Transform,},
    },
};

use rayon::prelude::*;

use crate::game::{
    tile::Tile,
};


#[derive(SystemDesc, Default)]
pub struct TileEditSystem {
    pub pressed_up: bool,
    pub pressed_down: bool,
}

impl <'s> System<'s> for TileEditSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Tile>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, LazyUpdate>
    );

    fn run(&mut self, (entities, mut tiles, mut transforms, mut sprite_renders, input, updater): Self::SystemData) {
        for (tile, transform, sprite_render) in (&mut tiles, &transforms, &mut sprite_renders).join() {
            if tile.selected {
                if tile.is_select_entity_created {
                    let holding_up = input.action_is_down("up").unwrap_or(false);
                    let holding_down = input.action_is_down("down").unwrap_or(false);

                    
                }
                else {
                    let highlight = entities.create();

                    let mut highlight_sprite_render = sprite_render.clone();
                    highlight_sprite_render.sprite_number = 0;
                    
                    let mut highlight_transform = transform.clone();
                    highlight_transform.set_translation_z(highlight_transform.translation().z + 1.0);
                    
                    updater.insert(
                        highlight,
                        highlight_sprite_render,
                    );

                    updater.insert(
                        highlight,
                        highlight_transform,
                    );

                    tile.is_select_entity_created = true;
                }
            }
        }

        // for (tile, transform, sprite_render) in (&tiles, &transforms, &mut sprite_renders).join() {
        //     let highlight = entities.create();

        //     if tile.selected {

        //     }
        //     else if !tile.selected {

        //     }

        //     // if tile.selected {
        //     //     if self.created_entity {
        //     //         let holding_up = input.action_is_down("up").unwrap_or(false);
        //     //         let holding_down = input.action_is_down("down").unwrap_or(false);

        //     //         if !self.pressed_up && !holding_down && holding_up  {
        //     //             self.pressed_up = true;

        //     //             let target_sprite_number = sprite_render.sprite_number + 1;

        //     //             if target_sprite_number < 3 {
        //     //                 sprite_render.sprite_number = target_sprite_number;
        //     //             }
        //     //             else {
        //     //                 sprite_render.sprite_number = 1;
        //     //             }
        //     //         }
        //     //         else if !holding_up {
        //     //             self.pressed_up = false;
        //     //         }

        //     //         if !self.pressed_down && !holding_up && holding_down {
        //     //             self.pressed_down = true;

        //     //             let target_sprite_number = sprite_render.sprite_number - 1;

        //     //             if target_sprite_number as i32 > 0 as i32 {
        //     //                 sprite_render.sprite_number = target_sprite_number;
        //     //             }
        //     //             else {
        //     //                 sprite_render.sprite_number = 3;
        //     //             }
        //     //         }
        //     //         else if !holding_down {
        //     //             self.pressed_down = false;
        //     //         }
        //     //     }
        //     //     else {
        //     //         let mut clone_sprite_render = sprite_render.clone();
        //     //         clone_sprite_render.sprite_number = 0;
        //     //         updater.insert(
        //     //             highlight,
        //     //             sprite_render.clone()
        //     //         );

        //     //         updater.insert(
        //     //             highlight,
        //     //             transform.clone()
        //     //         );

        //     //         self.created_entity = true;
        //     //     }
        //     // }
        //     // else if !tile.selected && self.created_entity {
        //     //     let _ = entities.delete(highlight);

        //     //     self.created_entity = false;
        //     // }
        // }
    }
}