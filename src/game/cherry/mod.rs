use crate::game::cherry::resources::CherrySpawnTimer;
use crate::game::cherry::systems::{
    spawn_cherries, spawn_cherries_over_time, tick_cherry_spawn_timer,
};
use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;
pub const NUMBER_OF_CHERRIES: usize = 3;
pub const CHERRY_SIZE: f32 = 64.0; // This is the star sprite size.

pub struct CherryPlugin;

impl Plugin for CherryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CherrySpawnTimer>()
            .add_startup_system(spawn_cherries)
            .add_systems(
                (tick_cherry_spawn_timer, spawn_cherries_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
