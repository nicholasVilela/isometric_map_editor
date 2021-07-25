use amethyst::{
    prelude::*,
    utils::{
        application_root_dir,
        fps_counter::FpsCounterBundle,
    },
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::{
        transform::{
            TransformBundle,
        },
    },
    input::{InputBundle, StringBindings},
};

mod systems;

mod game;
use game::GameState;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_display_dir = app_root.join("config/display.ron");
    let config_binding_dir = app_root.join("config/bindings.ron");
    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(config_display_dir)?
                        .with_clear([0.01, 0.01, 0.01, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(config_binding_dir)?)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with(systems::CameraMovementSystem::default(), "camera_movement_system", &["input_system",])
        .with(systems::ActionControllerSystem::default(), "action_controller_system", &["input_system",]);

    let mut game = Application::new(assets_dir, GameState::default(), game_data)?;

    game.run();

    Ok(())
}