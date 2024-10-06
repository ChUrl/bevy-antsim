use crate::{
    components::common::{
        AnimationIndices, AnimationTimer, Position, RandomizedVelocityChange, Velocity,
        VelocityChangeTimer,
    },
    ANT_SPEED, MAX_POSITION, MIN_POSITION, VELOCITY_CHANGE_MAX, VELOCITY_CHANGE_SCALE,
};
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

/// Randomizes the ``RandomizeVelocityChange`` components.
#[allow(clippy::needless_pass_by_value)]
pub fn randomized_velocity_change_system(
    mut query: Query<(&mut RandomizedVelocityChange, &mut VelocityChangeTimer)>,
    time: Res<Time>,
) {
    for (mut velocity_change, mut timer) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            velocity_change.0 += rand::thread_rng().gen_range(-1.0..1.0) * VELOCITY_CHANGE_SCALE;
            velocity_change.0 = velocity_change
                .0
                .clamp(-VELOCITY_CHANGE_MAX, VELOCITY_CHANGE_MAX);
        }
    }
}

/// Updates the ``Velocity`` components for entities that have a ``RandomizedVelocityChange``.
pub fn randomized_velocity_system(
    mut query: Query<(&mut Transform, &mut Velocity, &RandomizedVelocityChange)>,
) {
    for (mut transform, mut velocity, velocity_change) in &mut query {
        velocity.0 += velocity_change.0;
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., velocity.0 - PI / 2.);
    }
}

/// Updates ``Position`` and ``Transform`` components of entities that also have
/// the ``RandomizedVelocity`` component, like an ``Ant``.
pub fn position_update_system(
    // We query for each entity that has a Position, Transform and Velocity component.
    // The Transform component comes from the SpriteBundle.
    mut query: Query<(&mut Position, &mut Transform, &Velocity)>,
) {
    for (mut position, mut transform, velocity) in &mut query {
        let new_position: Vec2 = position.0 + Vec2::from_angle(velocity.0).normalize() * ANT_SPEED;

        transform.translation = new_position.clamp(MIN_POSITION, MAX_POSITION).extend(0.);
        position.0 = new_position.clamp(MIN_POSITION, MAX_POSITION);
    }
}

/// Sets an ant's velocity perpendicular to a wall, if touched.
pub fn wall_avoidance_system(mut query: Query<(&Position, &mut Velocity)>) {
    let touches_left_wall = |position: Vec2| -> bool { position.x <= MIN_POSITION.x };
    let touches_right_wall = |position: Vec2| -> bool { position.x >= MAX_POSITION.x };
    let touches_bottom_wall = |position: Vec2| -> bool { position.y <= MIN_POSITION.y };
    let touches_top_wall = |position: Vec2| -> bool { position.y >= MAX_POSITION.y };

    for (position, mut velocity) in &mut query {
        let mut new_velocity: f32 = velocity.0;

        if touches_left_wall(position.0) {
            new_velocity = 0.;
        } else if touches_right_wall(position.0) {
            new_velocity = PI;
        } else if touches_bottom_wall(position.0) {
            new_velocity = PI / 2.;
        } else if touches_top_wall(position.0) {
            new_velocity = 3. * PI / 2.;
        }

        velocity.0 = new_velocity;
    }
}

/// Animates the ants by incrementing their ``TextureAtlas`` indices, based on a timer.
#[allow(clippy::needless_pass_by_value)]
pub fn animation_system(
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    time: Res<Time>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
