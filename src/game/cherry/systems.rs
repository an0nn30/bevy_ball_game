use crate::game::cherry::components::Cherry;
use crate::game::cherry::resources::CherrySpawnTimer;
use crate::game::cherry::NUMBER_OF_CHERRIES;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub fn spawn_cherries(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_CHERRIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/cherry.png"),
                ..default()
            },
            Cherry {},
        ));
    }
}

pub fn tick_cherry_spawn_timer(mut cherry_spawn_timer: ResMut<CherrySpawnTimer>, time: Res<Time>) {
    cherry_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_cherries_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    cherry_spawn_timer: Res<CherrySpawnTimer>,
    cherry_query: Query<Entity, With<Cherry>>,
) {
    if cherry_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/cherry.png"),
                ..default()
            },
            Cherry {},
        ));
    }
}
