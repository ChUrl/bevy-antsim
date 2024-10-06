use super::common::{
    AnimationIndices, AnimationTimer, Position, RandomizedVelocityChange, Velocity,
    VelocityChangeTimer,
};
use crate::{ANT_ANIMATION_SPEED, VELOCITY_CHANGE_PERIOD, VELOCITY_CHANGE_SCALE};
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

/// This bundle represents the ant entity.
/// The ``state`` determines the ant's behavior, it stores its current ``position``
/// that is updated based on the ``randomized_velocity``.
/// The ``sprite`` is this entity's visual representation.
// Instead of calling commands.spawn(AntState::Searching, Position(Vec2::new(...)), ...),
// we can bundle-up the components that make up an Ant and simplify the spawning by
// using "impl" to define a "constructor"
#[derive(Bundle)]
#[allow(clippy::module_name_repetitions)]
pub struct AntBundle {
    pub state: AntState,
    pub position: Position,
    pub velocity: Velocity,
    pub velocity_change: RandomizedVelocityChange,
    // TODO: This timer is only needed once
    pub velocity_change_timer: VelocityChangeTimer,
    pub texture_atlas: TextureAtlas,
    pub animation_indices: AnimationIndices,
    // TODO: This timer is only needed once
    pub animation_timer: AnimationTimer,

    // These are not components, but other bundles of components. Those can be nested.
    pub sprite: SpriteBundle,
}

/// The states an ``Ant`` can assume.
#[derive(Component)]
#[allow(clippy::module_name_repetitions)]
pub enum AntState {
    /// The ant performs a random walk, searching for food.
    /// While walking, it drops Anthill-pheromones, leading back to the hill.
    /// Once food is found, the ant gets the Returning state.
    /// If the ant finds Food-pheromones, it gets the Fetching state.
    Searching,

    /// The ant found Food-pheromones and follows them to the food.
    /// Once the food is reached, the ant gets the Returning state.
    Fetching,

    /// The ant found food an is walking the path marked by the Anthill-pheromones.
    /// While returning, it drops Food-pheromones, leading to the food.
    /// Once the ant dropped off the food, it gets the Searching state.
    Returning,
}

// The "impl" keyword allows us to implement functions in a struct's namespace, i.e. "methods"
impl AntBundle {
    /// Instantiate a new ``AntBundle`` with a color and a starting position.
    pub fn new(
        position: Vec2,
        texture: Handle<Image>,
        texture_atlas: TextureAtlas,
        animation_indices: AnimationIndices,
        scale: Vec2,
    ) -> Self {
        Self {
            state: AntState::Searching,
            position: Position(position),
            velocity: Velocity(rand::thread_rng().gen_range(-PI..PI)),
            velocity_change: RandomizedVelocityChange(
                rand::thread_rng().gen_range(-1.0..1.0) * VELOCITY_CHANGE_SCALE,
            ),
            velocity_change_timer: VelocityChangeTimer(Timer::from_seconds(
                VELOCITY_CHANGE_PERIOD,
                TimerMode::Repeating,
            )),
            texture_atlas,
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(
                ANT_ANIMATION_SPEED,
                TimerMode::Repeating,
            )),
            sprite: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.),
                    scale: scale.extend(1.),
                    ..default()
                },
                texture,
                ..default()
            },
        }
    }
}
