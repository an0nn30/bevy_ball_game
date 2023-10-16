use crate::game::heart::resources::HeartSpawnTimer;
use crate::game::heart::systems::{spawn_hearts, spawn_hearts_over_time, tick_heart_spawn_timer};
use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;
pub const NUMBER_OF_HEARTS: usize = 3;
pub const HEART_SIZE: f32 = 16.0; // This is the star sprite size.

pub struct HeartPlugin;

impl Plugin for HeartPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeartSpawnTimer>()
            .add_startup_system(spawn_hearts)
            .add_systems(
                (tick_heart_spawn_timer, spawn_hearts_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
