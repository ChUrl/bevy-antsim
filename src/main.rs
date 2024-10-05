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
const ANT_SPEED: f32 = 0.25;
const RANDOM_WALK_CONE: f32 = PI / 180. * 20.;

/// The app's entrypoint.
fn main() {
    let mut app = App::new();

    // The DefaultPlugins contain the "Window" plugin.
    app.add_plugins(DefaultPlugins);

    // Sets the color used to clear the screen, i.e. the background color.
    app.insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)));

    // Startup systems are run once on startup.
    // The chain() function guarantees execution in the declared order.
    app.add_systems(Startup, (setup_system, hello_ants_system).chain());

    // Update systems are ran each update cycle, i.e. each frame.
    app.add_systems(
        Update,
        (
            randomized_velocity_system,
            wall_avoidance_system,
            random_walk_system,
        )
            .chain(),
    );

    app.run();
}
