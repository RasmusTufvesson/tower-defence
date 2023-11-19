use std::fs;

use bevy::prelude::*;
use serde::Deserialize;

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
    sprite: String,
    attack_speed: f32,
    attack: String,
}

#[derive(Resource, Deserialize)]
struct EnemyTemplate {
    sprite: String,
    health: f32,
    speed: f32,
    damage: f32,
}

#[derive(Resource, Deserialize)]
struct AttackTemplate {
    sprite: String,
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
    damage: f32,
}

fn spawn_tower(commands: &mut Commands, asset_server: &Res<AssetServer>, template: TowerTemplate, transform: Transform) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(&template.sprite),
            transform: transform,
            ..default()
        },
        Tower {
            attack: ron::from_str(&fs::read_to_string("assets/".to_string()+&template.attack).unwrap()).unwrap(), attack_speed: template.attack_speed
        },
    ));
}

fn spawn_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, template: EnemyTemplate, transform: Transform) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(&template.sprite),
            transform: transform,
            ..default()
        },
        Enemy {
            health: template.health, speed: template.speed, damage: template.damage
        },
    ));
}

fn get_enemy_template(path: &str) -> EnemyTemplate {
    ron::from_str(&fs::read_to_string("assets/".to_string()+path).unwrap()).unwrap()
}

fn get_tower_template(path: &str) -> TowerTemplate {
    ron::from_str(&fs::read_to_string("assets/".to_string()+path).unwrap()).unwrap()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let enemy = get_enemy_template("enemies/basic.ron");
    spawn_enemy(&mut commands, &asset_server, enemy, Transform { translation: Vec3 { x: 100.0, y: 100.0, z: 0.0 }, scale: Vec3 { x: 10.0, y: 10.0, z: 1.0}, ..default() })
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