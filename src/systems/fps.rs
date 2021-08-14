use amethyst::{
    derive::SystemDesc,
    ecs::{WriteStorage, ReadStorage, System, SystemData, Read, Join},
    utils::{
        fps_counter::FpsCounter
    },
    ui::{UiText, UiTransform}
};


#[derive(SystemDesc, Default)]
pub struct FpsSystem;

impl <'s> System<'s> for FpsSystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        ReadStorage<'s, UiTransform>,
        Read<'s, FpsCounter>,
    );

    fn run(&mut self, (mut texts, transforms, fps): Self::SystemData) {
        for (ui_text, transform) in (&mut texts, &transforms).join() {
            if transform.id == "fps" {
                ui_text.text = (fps.frame_fps() as i64).to_string();
            }
        }
    }
}