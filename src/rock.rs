use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{BlockType, Collectible, StaticCollisionCircle};

#[derive(Component)]
pub struct Rock {}

// pub fn unload_far_rocks(
//     mut commands: Commands,
//     rocks: Query<(&Collectible, Entity), With<Rock>>,
//     mut chunks_to_save: ResMut<ChunksToSave>,
//     player: Query<&Transform, With<Player>>,
// ) {
//     let player_translation = player.get_single().unwrap().translation;
//     for (collectible, entity) in &rocks {
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
//         let rocks_string = make_string_rep_of_rock(collectible);
//         // let mut rocks_string = String::new();
//         // rocks_string.push_str("rock ");
//         // rocks_string.push_str(collectible.pos.x.to_string().as_str());
//         // rocks_string.push(' ');
//         // rocks_string.push_str(collectible.pos.y.to_string().as_str());
//         // rocks_string.push(' ');
//         // rocks_string.push_str(collectible.uses.to_string().as_str());
//         // rocks_string.push('\n');

//         if let Some(rock_str) = chunks_to_save.chunks.get_mut(&pos) {
//             rock_str.push_str(rocks_string.as_str());
//         } else {
//             chunks_to_save.chunks.insert(pos, rocks_string);
//         }

//         commands.entity(entity).despawn();
//     }
// }

// pub fn save_all_rocks(
//     rocks: Query<&Collectible, With<Rock>>,
//     mut chunks_to_save: ResMut<ChunksToSave>,
// ) {
//     for collectible in &rocks {
//         let rocks_string = make_string_rep_of_rock(collectible);
//         // let mut rocks_string = String::new();
//         // rocks_string.push_str("rock ");
//         // rocks_string.push_str(collectible.pos.x.to_string().as_str());
//         // rocks_string.push(' ');
//         // rocks_string.push_str(collectible.pos.y.to_string().as_str());
//         // rocks_string.push(' ');
//         // rocks_string.push_str(collectible.uses.to_string().as_str());
//         // rocks_string.push('\n');

//         if let Some(rock_str) = chunks_to_save.chunks.get_mut(&(
//             (collectible.pos.x / 16.).floor() as i32,
//             (collectible.pos.y / 16.).floor() as i32,
//         )) {
//             rock_str.push_str(rocks_string.as_str());
//         } else {
//             chunks_to_save.chunks.insert(
//                 (
//                     (collectible.pos.x / 16.).floor() as i32,
//                     (collectible.pos.y / 16.).floor() as i32,
//                 ),
//                 rocks_string,
//             );
//         }
//     }
// }

// pub fn despawn_all_rocks(mut commands: Commands, rocks: Query<Entity, With<Rock>>) {
//     for entity in &rocks {
//         commands.entity(entity).despawn();
//     }
// }

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
            block_type: BlockType::Stone(0),
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
    spawn_rock(commands, meshes, materials, Vec3::new(x, y, 0.06), uses);
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
