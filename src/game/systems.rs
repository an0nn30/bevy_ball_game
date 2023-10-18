use crate::game::SimulationState;
use bevy::prelude::*;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_app_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if simulation_state.0 == SimulationState::Running {
            next_app_state.set(SimulationState::Paused);
            println!("Simulation Paused");
        }
        if simulation_state.0 == SimulationState::Paused {
            next_app_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
    }
}
