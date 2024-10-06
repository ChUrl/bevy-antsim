use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::{
        ant::AntBundle,
        anthill::{AntCapacity, GroupSize, SpawnTimer},
        common::AnimationIndices,
    },
    ANT_COUNT, ANT_SCALE,
};

#[allow(clippy::needless_pass_by_value)]
pub fn leave_anthill_system(
    mut query: Query<(&mut AntCapacity, &GroupSize, &mut SpawnTimer)>,
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (mut capacity, group_size, mut timer) in &mut query {
        timer.0.tick(time.delta());

        // Anthill empty :(
        if capacity.0 == 0 {
            continue;
        }

        // Each timer period we want to spawn between 1 and group_size ants
        if timer.0.just_finished() {
            let num_ants: u32 = rand::thread_rng()
                .gen_range(1..group_size.0)
                .clamp(1, capacity.0);

            for _ in 0..num_ants {
                spawn_cute_ant(&mut commands, &asset_server, &mut texture_atlas_layouts);
            }

            capacity.0 -= num_ants;
            println!(
                "Spawned {} cute ants 😍, {} ants are still at home 🏠",
                num_ants, capacity.0
            );
        }
    }
}

fn spawn_cute_ant(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs
    let texture: Handle<Image> = asset_server.load("sprites/ant_walk_anim.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(202, 248), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 61 };
    let texture_atlas = TextureAtlas {
        layout: texture_atlas_layout,
        index: rand::thread_rng().gen_range(animation_indices.first..=animation_indices.last),
    };

    commands.spawn(AntBundle::new(
        Vec2::ZERO,
        texture,
        texture_atlas,
        animation_indices,
        Vec2::splat(ANT_SCALE),
    ));
}

pub fn enter_anthill_system() {}
