use crate::{components::ant::AntBundle, ANT_COUNT};
use bevy::prelude::*;

/// Signal that the app has started by printing a message to the terminal.
pub fn hello_ants_system() {
    println!("Hello, fellow 🐜");
}

/// Prepares the environment before the Update schedule by spawning required entities.
pub fn setup_system(mut commands: Commands) {
    // Using the default camera, world space coordinates correspond 1:1 with screen pixels.
    // The point (0, 0) is in the center of the screen.
    commands.spawn(Camera2dBundle::default());

    // Spawn cute ants
    for _ in 0..ANT_COUNT {
        commands.spawn(AntBundle::new(Vec2::new(0., 0.), Color::srgb(1., 0., 0.)));
    }
    println!("Spawned {ANT_COUNT} cute ants 😍");
}
