use amethyst::{
    ecs::{WriteStorage, ReadStorage, System, SystemData, Join, Read,},
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
        ReadStorage<'s, Tile>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (tiles, mut transforms, mut sprite_renders, input): Self::SystemData) {
        for (tile, transform, sprite_render) in (&tiles, &transforms, &mut sprite_renders).join() {
            if tile.selected {
                let holding_up = input.action_is_down("up").unwrap_or(false);
                let holding_down = input.action_is_down("down").unwrap_or(false);

                if !self.pressed_up && !holding_down && holding_up  {
                    self.pressed_up = true;
                    self.pressed_down = false;

                    let target_sprite_number = sprite_render.sprite_number + 1;

                    if target_sprite_number < 3 {
                        sprite_render.sprite_number = target_sprite_number;
                    }
                    else {
                        sprite_render.sprite_number = 0;
                    }
                }
                else if !holding_up {
                    self.pressed_up = false;
                }

                if !self.pressed_down && !holding_up && holding_down {
                    self.pressed_down = true;
                    self.pressed_up = false;

                    let target_sprite_number = sprite_render.sprite_number - 1;

                    if target_sprite_number as i32 > -1 as i32 {
                        sprite_render.sprite_number = target_sprite_number;
                    }
                    else {
                        sprite_render.sprite_number = 3;
                    }
                }
                else if !holding_down {
                    self.pressed_down = false;
                }
            }
        }
    }
}