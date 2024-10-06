mod components;
mod systems;

use bevy::prelude::*;
use std::f32::consts::PI;
use systems::{
    ant::{
        animation_system, position_update_system, randomized_velocity_change_system,
        randomized_velocity_system, wall_avoidance_system,
    },
    startup::{hello_ants_system, setup_system},
};

// TODO: Use Settings Resource
const MIN_POSITION: Vec2 = Vec2::splat(-450.);
const MAX_POSITION: Vec2 = Vec2::splat(450.);

const ANT_COUNT: u32 = 200;
const ANT_SPEED: f32 = 0.5;
const ANT_ANIMATION_SPEED: f32 = 1. / 62.;
const ANT_SCALE: f32 = 0.15;

const VELOCITY_CHANGE_SCALE: f32 = PI / 180. * 1.; // Single degree
const VELOCITY_CHANGE_MAX: f32 = PI / 180. * 0.5;
const VELOCITY_CHANGE_PERIOD: f32 = 0.5;

/// The app's entrypoint.
fn main() {
    let mut app = App::new();

    // The DefaultPlugins contain the "Window" plugin.
    // ImagePlugin::default_nearest() is supposed to prevent blurry sprites.
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    // Sets the color used to clear the screen, i.e. the background color.
    app.insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)));

    // Sets the FixedUpdate frequency to 60hz.
    app.insert_resource(Time::<Fixed>::from_hz(60.));

    // Startup systems are ran once on startup.
    // The chain() function guarantees execution in the declared order.
    app.add_systems(Startup, (setup_system, hello_ants_system).chain());

    // FixedUpdate systems are ran at a fixed frequency.
    // They might be ran multiple times per frame to catch up, or be skipped.
    app.add_systems(
        FixedUpdate,
        (
            randomized_velocity_change_system,
            randomized_velocity_system,
            wall_avoidance_system,
            position_update_system,
            animation_system,
        )
            .chain(),
    );

    app.run();
}
