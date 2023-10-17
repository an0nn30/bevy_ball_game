use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use crate::game::player::resources::GodModeTimer;
use crate::game::SimulationState;
use crate::AppState;
use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerState>()
            .init_resource::<GodModeTimer>()
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_startup_system(spawn_player)
            .add_system(tick_god_mode_timer.run_if(in_state(PlayerState::God)))
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                    enemy_hit_player,
                    player_hit_star,
                    player_hit_cherry,
                    toggle_god_mode_timer,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerState {
    #[default]
    Normal,
    God,
}
