use amethyst::{
    prelude::*,
    utils::{
        application_root_dir,
        fps_counter::{
            FpsCounter
        }
    },
    ui::{UiText, FontAsset, LineMode, Anchor, TtfFormat, UiTransform},
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
        SpriteRender,
    },
};

#[path = "./tile.rs"]
pub mod tile;
pub use tile::Tile;

#[path = "./util.rs"]
pub mod util;
pub use util::{
    map_to_screen,
    screen_to_map,
    get_map_dimensions, 
    get_tile_dimensions, 
    get_window_dimensions,
    config::{GameConfig, TileConfig},
};


#[derive(Default)]
pub struct GameState {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.sprite_sheet_handle
            .replace(load_sprite_sheet(world));

        world.insert(self.sprite_sheet_handle.clone());

        setup_config(world);
        setup_fps_counter(world);
        setup_camera(world);
        setup_map(world, self.sprite_sheet_handle.clone().unwrap());
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        return Trans::None;
    }
}


fn setup_config(world: &mut World) {
    let app_root = application_root_dir().unwrap();
    let config_path = app_root.join("config/config.ron");
    let config = GameConfig::load(&config_path).unwrap();

    world.insert(config);
}

fn setup_fps_counter(world: &mut World) {
    world.insert(FpsCounter::default());
    world.insert(AssetStorage::<FontAsset>::default());


    let font_handle = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let ui_transform = UiTransform::new(
        String::from("fps"),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0f32,
        0f32,
        0f32,
        40f32,
        30f32,
    );

    world.create_entity()
        .with(UiText::new(
            font_handle,
            String::from(""),
            [1.0, 1.0, 1.0, 1.0],
            25f32,
            LineMode::Single,
            Anchor::Middle
        ))
        .with(ui_transform)
        .build();
}

fn setup_camera(world: &mut World) {
    let (window_height, window_width) = get_window_dimensions(world);

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 100.0);

    world.create_entity()
        .with(Camera::standard_2d(window_width, window_height))
        .with(transform)
        .build();
}

fn setup_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {  
    let tile_render = SpriteRender::new(sprite_sheet_handle, 0);

    let (map_height, map_width) = get_map_dimensions(world);
    let tile = get_tile_config(world);

    let font_handle = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    for y in 0..map_height.floor() as isize {
        for x in 0..map_width.floor() as isize {
            let mut transform = Transform::default();

            let tile_iso_position = map_to_screen(x as i32, y as i32, tile.width, tile.height);
            transform.set_translation_xyz(tile_iso_position.x, tile_iso_position.y, x as f32 + y as f32);

            let ui_text = UiText::new(
                font_handle.clone(),
                String::from(""),
                [1.0, 1.0, 1.0, 1.0],
                15f32,
                LineMode::Single,
                Anchor::Middle
            );

            let ui_transform = UiTransform::new(
                format!("tile({:?},{:?})", x, y),
                Anchor::Middle,
                Anchor::Middle,
                0f32,
                0f32,
                0f32,
                100f32,
                100f32,
            );

            world.create_entity()
                .with(Tile::new(x, y))
                .with(tile_render.clone())
                .with(transform)
                .with(ui_text)
                .with(ui_transform)
                .build();
        }
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "textures/sprite_sheet_3.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    return loader.load(
        "textures/sprite_sheet_3.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn get_tile_config(world: &mut World) -> TileConfig {
    let config = world.read_resource::<GameConfig>();

    return config.tile;
}