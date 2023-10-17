use bevy::prelude::*;

pub const GOD_MODE_TIMER: f32 = 3.0;
#[derive(Resource)]
pub struct GodModeTimer {
    pub timer: Timer,
}

impl Default for GodModeTimer {
    fn default() -> GodModeTimer {
        GodModeTimer {
            timer: Timer::from_seconds(GOD_MODE_TIMER, TimerMode::Repeating),
        }
    }
}
