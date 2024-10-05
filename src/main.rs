mod components;
mod systems;

use bevy::prelude::*;
use std::f32::consts::PI;
use systems::{
    random_walk::{random_walk_system, randomized_velocity_system, wall_avoidance_system},
    startup::{hello_ants_system, setup_system},
};

// TODO: Use Settings Resource
const MIN_POSITION: Vec2 = Vec2::ZERO;
const MAX_POSITION: Vec2 = Vec2::new(500., 500.);

const ANT_COUNT: u32 = 25;
const ANT_SPEED: f32 = 0.75;
const RANDOM_WALK_CONE: f32 = PI / 180. * 20.;

/// The app's entrypoint.
fn main() {
    let mut app = App::new();

    // The DefaultPlugins contain the "Window" plugin.
    app.add_plugins(DefaultPlugins);

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
            randomized_velocity_system,
            wall_avoidance_system,
            random_walk_system,
        )
            .chain(),
    );

    app.run();
}
