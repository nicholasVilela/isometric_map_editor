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
    core::{
        transform::Transform,
        timing::Time,
    },
    input::{
        InputHandler,
        StringBindings,
    },
};


#[derive(SystemDesc)]
pub struct CameraMovementSystem {
    pub move_speed: f32,
}

impl Default for CameraMovementSystem {
    fn default() -> Self {
        return CameraMovementSystem {
            move_speed: 30.0
        };
    }
}

impl <'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, cameras, input, time): Self::SystemData) {
        for (transform, _) in (&mut transforms, &cameras).join() {
            let middle_down = input.action_is_down("mouse_middle").unwrap_or(false);

            if middle_down {
                let d_x = input.axis_value("mouse_x").unwrap_or(0.0);
                let d_y = input.axis_value("mouse_y").unwrap_or(0.0);

                let x = d_x * self.move_speed * time.delta_seconds();
                let y = -d_y * self.move_speed * time.delta_seconds();

                transform.move_right(x);
                transform.move_up(y);
            }
        }
    }
}