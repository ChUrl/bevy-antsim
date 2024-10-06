use crate::{
    components::{ant::AntBundle, common::AnimationIndices},
    ANT_COUNT, ANT_SCALE, MAX_POSITION,
};
use bevy::prelude::*;
use rand::Rng;

/// Signal that the app has started by printing a message to the terminal.
pub fn hello_ants_system() {
    println!("Hello, fellow 🐜");
}

/// Prepares the environment before the Update schedule by spawning required entities.
#[allow(clippy::needless_pass_by_value)]
pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Using the default camera, world space coordinates correspond 1:1 with screen pixels.
    // The point (0, 0) is in the center of the screen.
    commands.spawn(Camera2dBundle::default());

    // Spawn cute ants
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs
    // TODO: Some weird cloning here
    let texture: Handle<Image> = asset_server.load("sprites/ant_walk_anim.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(202, 248), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    for _ in 0..ANT_COUNT {
        let animation_indices = AnimationIndices { first: 0, last: 61 };
        let texture_atlas = TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: rand::thread_rng().gen_range(animation_indices.first..=animation_indices.last),
        };

        commands.spawn(AntBundle::new(
            Vec2::ZERO,
            texture.clone(),
            texture_atlas,
            animation_indices,
            Vec2::splat(ANT_SCALE),
        ));
    }
    println!("Spawned {ANT_COUNT} cute ants 😍");
}
