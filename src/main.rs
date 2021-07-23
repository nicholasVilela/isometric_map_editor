use amethyst::{
    prelude::*,
    utils::application_root_dir,
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
    // input::{InputBundle, StringBindings},
};


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_display_dir = app_root.join("config/display.ron");
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
        .with_bundle(TransformBundle::new())?;

    Ok(())
}