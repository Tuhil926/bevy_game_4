use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{BlockType, Collectible, ItemType, StaticCollisionCircle};

#[derive(Component)]
pub struct Rock {}

pub fn spawn_rock(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    // asset_server: &Res<AssetServer>,
    pos: Vec3,
    uses: usize,
) {
    // let mut rng = rand::thread_rng();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(1.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.4, 0.4, 0.4))),
            transform: Transform::from_translation(pos),
            ..default()
        },
        // SpriteBundle {
        //     sprite: Sprite {
        //         custom_size: Some(Vec2::new(2., 2.)),
        //         ..default()
        //     },
        //     transform: Transform::from_translation(pos)
        //         .with_rotation(Quat::from_rotation_z(rng.gen::<f32>() * 6.28))
        //         .with_scale(Vec3::splat(0.5 + rng.gen::<f32>())),
        //     texture: asset_server.load("rock.png"),
        //     ..default()
        // },
        Rock {},
        Collectible {
            pos: Vec2::new(pos.x, pos.y),
            gather_radius: 3.,
            item_type: ItemType::Stone(0),
            uses: uses,
        },
        StaticCollisionCircle { radius: 1. },
    ));
}

pub fn make_string_rep_of_rock(collectible: &Collectible) -> String {
    let mut rocks_string = String::new();
    rocks_string.push_str("rock ");
    rocks_string.push_str(collectible.pos.x.to_string().as_str());
    rocks_string.push(' ');
    rocks_string.push_str(collectible.pos.y.to_string().as_str());
    rocks_string.push(' ');
    rocks_string.push_str(collectible.uses.to_string().as_str());
    rocks_string.push('\n');
    rocks_string
}

pub fn spawn_rock_from_string_rep(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parts: Vec<&str>, // a vector of strings, containing the words in the string representation
) {
    let x = parts[1].parse::<f32>().unwrap();
    let y = parts[2].parse::<f32>().unwrap();
    let uses = parts[3].parse::<usize>().unwrap();
    spawn_rock(commands, meshes, materials, Vec3::new(x, y, 0.07), uses);
}

pub fn generate_rocks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    pos: (i32, i32),
) {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        if rng.gen::<f32>() < 0.5 {
            continue;
        }
        let pos = Vec3::new(
            ((rng.gen::<f32>()) + pos.0 as f32) * 16.,
            ((rng.gen::<f32>()) + pos.1 as f32) * 16.,
            0.07,
        );
        spawn_rock(commands, meshes, materials, pos, 10);
    }
}
