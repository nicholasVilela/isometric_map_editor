use amethyst::{
    prelude::*,
    utils::application_root_dir,
    core::{
        transform::Transform,
    },
    assets::{
        Handle,
        Loader,
        AssetStorage,
    },
    renderer::{
        Camera,
        SpriteSheet,
        ImageFormat,
        SpriteSheetFormat,
        Texture,
    }
};

#[path = "./config.rs"]
mod config;
use config::GameConfig;


#[derive(Default)]
pub struct GameState {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        self.sprite_sheet_handle
            .replace(load_sprite_sheet(world));

        setup_config(world);
        setup_camera(world);
        setup_map(world, self.sprite_sheet_handle.clone().unwrap());
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        return Trans::None;
    }
}


fn setup_config(world: &mut World) {
    let app_root = application_root_dir().unwrap();
    let config_path = app_root.join("config/config.ron");
    let config = GameConfig::load(&config_path).unwrap();

    world.insert(config);
}

fn setup_camera(world: &mut World) {
    let (map_height, map_width) = get_map_dimensions(world);
    let (tile_height, tile_width) = get_tile_dimensions(world);
    let board_pixel_width = map_width * tile_width;
    let board_pixel_height = map_height * tile_height;

    let mut transform = Transform::default();
    transform.set_translation_xyz(board_pixel_width * 0.5, board_pixel_height * 0.5, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(board_pixel_width, board_pixel_height))
        .with(transform)
        .build();
}

fn setup_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

}


fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "textures/sprite_sheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    return loader.load(
        "textures/sprite_sheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn get_map_dimensions(world: &mut World) -> (f32, f32) {
    let config = world.read_resource::<GameConfig>();

    return (config.map.height, config.map.width);
}

fn get_tile_dimensions(world: &mut World) -> (f32, f32) {
    let config = world.read_resource::<GameConfig>();

    return (config.tile.height, config.tile.width);
}