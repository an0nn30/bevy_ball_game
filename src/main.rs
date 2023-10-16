pub mod events;
mod systems;

mod game;
mod main_menu;

use bevy::prelude::*;
use systems::*;

use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(transition_to_main_menu)
        .add_system(transition_to_game_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
