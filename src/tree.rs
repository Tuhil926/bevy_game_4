use crate::{BlockType, Collectible, StaticCollisionCircle};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

#[derive(Component)]
pub struct Tree {}

// pub fn unload_far_trees(
//     mut commands: Commands,
//     trees: Query<(&Collectible, Entity), With<Tree>>,
//     mut chunks_to_save: ResMut<ChunksToSave>,
//     player: Query<&Transform, With<Player>>,
// ) {
//     let player_translation = player.get_single().unwrap().translation;
//     for (collectible, entity) in &trees {
//         let pos = (
//             (collectible.pos.x / 16.).floor() as i32,
//             (collectible.pos.y / 16.).floor() as i32,
//         );
//         let player_pos = (
//             (player_translation.x / 16.).floor() as i32,
//             (player_translation.y / 16.).floor() as i32,
//         );

//         if (pos.0 - player_pos.0).abs() <= 1 && (pos.1 - player_pos.1).abs() <= 1 {
//             continue;
//         }
//         let trees_string = make_string_rep_of_tree(collectible);

//         // let mut trees_string = String::new();
//         // trees_string.push_str("tree ");
//         // trees_string.push_str(collectible.pos.x.to_string().as_str());
//         // trees_string.push(' ');
//         // trees_string.push_str(collectible.pos.y.to_string().as_str());
//         // trees_string.push(' ');
//         // trees_string.push_str(collectible.uses.to_string().as_str());
//         // trees_string.push('\n');

//         if let Some(tree_str) = chunks_to_save.chunks.get_mut(&pos) {
//             tree_str.push_str(trees_string.as_str());
//         } else {
//             chunks_to_save.chunks.insert(pos, trees_string);
//         }

//         commands.entity(entity).despawn();
//     }
// }

// pub fn save_all_trees(
//     trees: Query<&Collectible, With<Tree>>,
//     mut chunks_to_save: ResMut<ChunksToSave>,
// ) {
//     for collectible in &trees {
//         let trees_string = make_string_rep_of_tree(collectible);
//         // let mut trees_string = String::new();
//         // trees_string.push_str("tree ");
//         // trees_string.push_str(collectible.pos.x.to_string().as_str());
//         // trees_string.push(' ');
//         // trees_string.push_str(collectible.pos.y.to_string().as_str());
//         // trees_string.push(' ');
//         // trees_string.push_str(collectible.uses.to_string().as_str());
//         // trees_string.push('\n');

//         if let Some(tree_str) = chunks_to_save.chunks.get_mut(&(
//             (collectible.pos.x / 16.).floor() as i32,
//             (collectible.pos.y / 16.).floor() as i32,
//         )) {
//             tree_str.push_str(trees_string.as_str());
//         } else {
//             chunks_to_save.chunks.insert(
//                 (
//                     (collectible.pos.x / 16.).floor() as i32,
//                     (collectible.pos.y / 16.).floor() as i32,
//                 ),
//                 trees_string,
//             );
//         }
//     }
// }
// pub fn despawn_all_trees(mut commands: Commands, trees: Query<Entity, With<Tree>>) {
//     for entity in &trees {
//         commands.entity(entity).despawn();
//     }
// }

pub fn spawn_tree(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec3,
    uses: usize,
) {
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
            block_type: BlockType::Wood,
            uses: uses,
        },
        StaticCollisionCircle { radius: 1. },
    ));
    println!("spawned a tree at {}, {}", pos.x, pos.y);
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
