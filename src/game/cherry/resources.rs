use bevy::prelude::*;

pub const CHERRY_SPAWN_TIME: f32 = 10.0;
#[derive(Resource)]
pub struct CherrySpawnTimer {
    pub timer: Timer,
}

impl Default for CherrySpawnTimer {
    fn default() -> CherrySpawnTimer {
        CherrySpawnTimer {
            timer: Timer::from_seconds(CHERRY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
