use amethyst::{
    derive::SystemDesc,
    ecs::{
        Join,
        WriteStorage,
        ReadStorage,
        SystemData,
        System,
        Read,
    },
    renderer::{
        Camera,
    },
    core::transform::Transform,
    input::{
        InputHandler,
        StringBindings,
    },
};


#[derive(SystemDesc, Default)]
pub struct CameraMovementSystem;

impl <'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cameras, input): Self::SystemData) {
        for (transform, camera) in (&mut transforms, &cameras).join() {
            let middle_down = input.action_is_down("mouse_middle").unwrap_or(false);

            if middle_down {
                let d_x = input.axis_value("mouse_x").unwrap_or(0.0);
                let d_y = input.axis_value("mouse_y").unwrap_or(0.0);

                transform.move_right(d_x);
                transform.move_up(-d_y);
            }
        }
    }
}