use crate::{Collectible, ItemType, StaticCollisionCircle};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

#[derive(Component)]
pub struct Tree {}

pub fn spawn_tree(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec3,
    uses: usize,
) {
    let normal_material = materials.add(ColorMaterial::from(Color::rgb(0.4, 1.0, 0.1)));
    let punched_material = materials.add(ColorMaterial::from(Color::rgb(0.8, 1.0, 0.5)));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(1.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.4, 1.0, 0.1))),
            transform: Transform::from_translation(pos),
            ..default()
        },
        Tree {},
        Collectible {
            pos: Vec2::new(pos.x, pos.y),
            gather_radius: 3.,
            item_type: ItemType::Wood,
            uses: uses,
            normal_material: normal_material,
            punched_material: punched_material,
        },
        StaticCollisionCircle { radius: 1. },
    ));
    // println!("spawned a tree at {}, {}", pos.x, pos.y);
}

pub fn make_string_rep_of_tree(collectible: &Collectible) -> String {
    let mut trees_string = String::new();
    trees_string.push_str("tree ");
    trees_string.push_str(collectible.pos.x.to_string().as_str());
    trees_string.push(' ');
    trees_string.push_str(collectible.pos.y.to_string().as_str());
    trees_string.push(' ');
    trees_string.push_str(collectible.uses.to_string().as_str());
    trees_string.push('\n');
    trees_string
}

pub fn spawn_tree_from_string_rep(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parts: Vec<&str>, // a vector of strings, containing the words in the string representation
) {
    let x = parts[1].parse::<f32>().unwrap();
    let y = parts[2].parse::<f32>().unwrap();
    let uses = parts[3].parse::<usize>().unwrap();
    spawn_tree(commands, meshes, materials, Vec3::new(x, y, 0.06), uses);
}

pub fn generate_trees(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
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
            0.06,
        );
        spawn_tree(commands, meshes, materials, pos, 10);
    }
}
