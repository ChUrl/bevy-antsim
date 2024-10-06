use crate::{components::anthill::AnthillBundle, ANT_COUNT};
use bevy::prelude::*;

/// Signal that the app has started by printing a message to the terminal.
pub fn hello_ants_system() {
    println!("Hello, fellow 🐜");
}

/// Prepares the environment before the Update schedule by spawning required entities.
#[allow(clippy::needless_pass_by_value)]
pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Using the default camera, world space coordinates correspond 1:1 with screen pixels.
    // The point (0, 0) is in the center of the screen.
    commands.spawn(Camera2dBundle::default());

    // Spawn the anthill that will spawn the ants
    commands.spawn(AnthillBundle::new(
        ANT_COUNT,
        5,
        1.,
        Vec2::ZERO,
        Vec2::splat(0.2),
        asset_server.load("sprites/anthill.png"),
    ));
}
