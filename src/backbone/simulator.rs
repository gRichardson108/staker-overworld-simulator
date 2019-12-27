use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::components::map::Zoomable;

/// default size of the map
/// TODO: make this a runtime resource
pub const MAP_HEIGHT: f32 = 2048.0;
pub const MAP_WIDTH: f32 = 1024.0;

/// Struct for primary simulation application state, such as when
/// the simulation is active and interactive
#[derive(Default)]
pub struct Simulator {
    map_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Simulator {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.map_sprite_sheet_handle.replace(load_map_sprite(world));

        world.register::<Zoomable>();

        initialize_map(world, self.map_sprite_sheet_handle.clone().unwrap());
        initialize_camera(world);
    }
}

/// Loads the map texture.
/// TODO: Make this configurable at runtime.
fn load_map_sprite(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        // load and return the texture handle
        loader.load(
            "texture/stalkerAnomalyMap.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/stalkerAnomalyMap.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_map(world: &mut World, map_sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut local_transform = Transform::default();

    // Assign the sprite for the map
    let sprite_render = SpriteRender {
        sprite_sheet: map_sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Zoomable {
            min_zoom: 0.0,
            max_zoom: 100.0,
            current_zoom: 50.0,
        })
        .with(local_transform)
        .build();
}

fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(MAP_WIDTH * 0.0, MAP_HEIGHT * 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(MAP_WIDTH, MAP_HEIGHT))
        .with(transform)
        .build();
}