use bevy::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        // .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Resource, Deserialize)]
struct TowerTemplate {
    sprite: &str,
    attack_speed: f32,
    attack: &str,
}

#[derive(Resource, Deserialize)]
struct EnemyTemplate {
    sprite: &str,
    health: f32,
    speed: f32,
}

#[derive(Resource, Deserialize)]
struct AttackTemplate {
    sprite: &str,
    damage: f32,
}

#[derive(Component)]
struct Tower {
    attack_speed: f32,
    attack: AttackTemplate,
}

#[derive(Component)]
struct Enemy {
    health: f32,
    speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(Camera2dBundle::default());
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("pixel/bevy_pixel_dark.png"),
    //         transform: Transform::from_xyz(100., 0., 0.),
    //         ..default()
    //     },
    //     Direction::Right,
    // ));
}

// fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
//     for (mut logo, mut transform) in &mut sprite_position {
//         match *logo {
//             Direction::Right => transform.translation.x += 30. * time.delta_seconds(),
//             Direction::Left => transform.translation.x -= 30. * time.delta_seconds(),
//         }

//         if transform.translation.x > 200. {
//             *logo = Direction::Left;
//         } else if transform.translation.x < -200. {
//             *logo = Direction::Right;
//         }
//     }
// }