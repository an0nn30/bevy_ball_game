use crate::game::heart::components::Heart;
use crate::game::heart::resources::HeartSpawnTimer;
use crate::game::heart::NUMBER_OF_HEARTS;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub fn spawn_hearts(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_HEARTS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/heart.png"),
                ..default()
            },
            Heart {},
        ));
    }
}

pub fn tick_heart_spawn_timer(mut star_spawn_timer: ResMut<HeartSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_hearts_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<HeartSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/heart.png"),
                ..default()
            },
            Heart {},
        ));
    }
}
