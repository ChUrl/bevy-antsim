use crate::{
    components::common::{Position, RandomizedVelocity},
    ANT_SPEED, MAX_POSITION, MIN_POSITION, RANDOM_WALK_CONE,
};
use bevy::prelude::*;

/// Randomizes the ``RandomizedVelocity`` components of entities that have them.
#[allow(clippy::needless_pass_by_value)] // I can't specify &Res<Time>, so pass by value
pub fn randomized_velocity_system(mut query: Query<&mut RandomizedVelocity>) {
    for mut velocity in &mut query {
        let mut new_velocity: Vec2 = Vec2::from_angle(
            // We first multiply with CONE to get the angle-delta to [0, CONE].
            // Then, we subtract CONE/2 to bring the angle-delta to [-CONE/2, CONE/2].
            // Lastly, we add the angle-delta to the current angle.
            // This should vary the direction in a front-facing CONE-degree cone.
            rand::random::<f32>().mul_add(
                RANDOM_WALK_CONE,
                velocity.0.to_angle() - RANDOM_WALK_CONE / 2.,
            ),
        );
        new_velocity = new_velocity.normalize() * ANT_SPEED;

        velocity.0 = new_velocity;
    }
}

/// Updates ``Position`` and ``Transform`` components of entities that also have
/// the ``RandomizedVelocity`` component, like an ``Ant``.
#[allow(clippy::module_name_repetitions)]
pub fn random_walk_system(
    // We query for each entity that has a Position, Transform and Velocity component.
    // The Transform component comes from the SpriteBundle.
    mut query: Query<(&mut Position, &mut Transform, &RandomizedVelocity)>,
) {
    for (mut position, mut transform, velocity) in &mut query {
        let new_position: Vec2 = position.0 + velocity.0;

        transform.translation = new_position.clamp(MIN_POSITION, MAX_POSITION).extend(0.);
        position.0 = new_position.clamp(MIN_POSITION, MAX_POSITION);
    }
}

/// Sets an ant's velocity perpendicular to a wall, if touched.
pub fn wall_avoidance_system(mut query: Query<(&Position, &mut RandomizedVelocity)>) {
    let touches_left_wall = |position: Vec2| -> bool { position.x <= 0. };
    let touches_right_wall = |position: Vec2| -> bool { position.x >= MAX_POSITION.x };
    let touches_bottom_wall = |position: Vec2| -> bool { position.y <= 0. };
    let touches_top_wall = |position: Vec2| -> bool { position.y >= MAX_POSITION.y };

    for (position, mut velocity) in &mut query {
        let mut new_velocity: Vec2 = velocity.0;

        if touches_left_wall(position.0) {
            new_velocity = Vec2::new(1., 0.);
        } else if touches_right_wall(position.0) {
            new_velocity = Vec2::new(-1., 0.);
        } else if touches_bottom_wall(position.0) {
            new_velocity = Vec2::new(0., 1.);
        } else if touches_top_wall(position.0) {
            new_velocity = Vec2::new(0., -1.);
        }

        velocity.0 = new_velocity;
    }
}
