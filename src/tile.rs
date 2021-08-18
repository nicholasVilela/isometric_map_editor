use amethyst::{
    ecs::{
        Component,
        VecStorage,
    },
};


#[derive(Debug)]
pub struct Tile {
    pub map_x: Option<isize>,
    pub map_y: Option<isize>,
    pub selected: bool,
    pub is_select_entity_created: bool,
}

impl Tile {
    pub fn new(x: isize, y: isize) -> Tile {
        return Tile {
            map_x: Some(x),
            map_y: Some(y),
            selected: false,
            is_select_entity_created: false,
        };
    }
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}