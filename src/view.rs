use amethyst::{
    prelude::*,
}


pub struct ViewState;

impl SimpleState for ViewState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
    }

    fn on_update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        return Trans::None;
    }
}