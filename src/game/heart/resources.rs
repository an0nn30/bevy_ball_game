use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 10.0;
#[derive(Resource)]
pub struct HeartSpawnTimer {
    pub timer: Timer,
}

impl Default for HeartSpawnTimer {
    fn default() -> HeartSpawnTimer {
        HeartSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
