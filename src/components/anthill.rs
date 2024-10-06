use bevy::prelude::*;

#[derive(Bundle)]
#[allow(clippy::module_name_repetitions)]
pub struct AnthillBundle {
    pub capacity: AntCapacity,
    pub group_size: GroupSize,
    pub spawn_timer: SpawnTimer,
    pub sprite: SpriteBundle,
}

#[derive(Component)]
pub struct AntCapacity(pub u32);

#[derive(Component)]
pub struct GroupSize(pub u32);

#[derive(Component)]
pub struct SpawnTimer(pub Timer);

impl AnthillBundle {
    pub fn new(
        ant_count: u32,
        group_size: u32,
        period: f32,
        position: Vec2,
        scale: Vec2,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            capacity: AntCapacity(ant_count),
            group_size: GroupSize(group_size),
            spawn_timer: SpawnTimer(Timer::from_seconds(period, TimerMode::Repeating)),
            sprite: SpriteBundle {
                transform: Transform {
                    translation: position.extend(1.), // Make sure the anthill occludes the ants
                    scale: scale.extend(1.),
                    ..default()
                },
                texture,
                ..default()
            },
        }
    }
}
