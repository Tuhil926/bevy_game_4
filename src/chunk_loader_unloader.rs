use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Resource)]
pub struct ChunksToSave {
    pub chunks: HashMap<(i32, i32), String>,
    pub chunks_loaded: HashSet<(i32, i32)>,
}

#[derive(Component)]
pub struct ChunkBackground {}

use crate::{
    generate_rocks, generate_trees, insert_block_to_inventory, make_string_rep_of_rock,
    make_string_rep_of_tree, spawn_block, spawn_rock_from_string_rep, spawn_tree_from_string_rep,
    BlockEntity, BlockType, Collectible, CollisionBox, Map, PhysicsBody, Player, PlayerInventory,
    Rock, Tree,
};

pub fn save_players(
    players: Query<(&PhysicsBody, &Transform), With<Player>>,
    inventory: Res<PlayerInventory>,
) {
    let mut players_string = String::new();
    for (physics_body, transform) in &players {
        players_string.push_str("pos ");
        players_string.push_str(transform.translation.x.to_string().as_str());
        players_string.push(' ');
        players_string.push_str(transform.translation.y.to_string().as_str());
        players_string.push(' ');
        players_string.push_str(physics_body.vel.x.to_string().as_str());
        players_string.push(' ');
        players_string.push_str(physics_body.vel.y.to_string().as_str());
        players_string.push(' ');
        players_string.push_str(physics_body.acc.x.to_string().as_str());
        players_string.push(' ');
        players_string.push_str(physics_body.acc.y.to_string().as_str());
        players_string.push('\n');
    }

    for i in 0..inventory.slots.len() {
        let mut block_type = None;
        let mut count = 0;

        match inventory.slots[i] {
            Some(slot) => {
                block_type = Some(slot.item_type);
                count = slot.count;
            }
            None => {}
        }

        players_string.push_str("slot ");
        players_string.push_str(get_string_rep_from_block_type(block_type).0.as_str());
        players_string.push(' ');
        players_string.push_str(count.to_string().as_str());
        players_string.push('\n');
    }

    fs::write("./assets/players.txt", players_string).expect("Could not save players!");
}

pub fn get_block_type_from_string_rep(block_type: &str, data: &[&str]) -> Option<BlockType> {
    dbg!(data);
    match block_type {
        "wood" => Some(BlockType::Wood),
        "stone" => Some(BlockType::Stone(if data.len() > 0 {
            data[0].parse::<i32>().unwrap()
        } else {
            0
        })),
        _ => None,
    }
}

pub fn get_string_rep_from_block_type(block_type: Option<BlockType>) -> (String, String) {
    match block_type {
        Some(block) => match block {
            BlockType::Wood => (String::from("wood"), String::new()),
            BlockType::Stone(number) => (String::from("stone"), format!("{}", number)),
        },
        None => (String::from("nothing"), String::new()),
    }
}

pub fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut inventory: ResMut<PlayerInventory>,
) {
    if let Ok(players_string) = fs::read_to_string("./assets/players.txt") {
        let lines = players_string.split('\n').collect::<Vec<&str>>();
        for line in lines {
            let parts = line.split(' ').collect::<Vec<&str>>();
            match parts[0] {
                "pos" => {
                    let x = parts[1].parse::<f32>().unwrap();
                    let y = parts[2].parse::<f32>().unwrap();
                    let vel_x = parts[3].parse::<f32>().unwrap();
                    let vel_y = parts[4].parse::<f32>().unwrap();
                    let acc_x = parts[5].parse::<f32>().unwrap();
                    let acc_y = parts[6].parse::<f32>().unwrap();

                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(0.5).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::rgb(0.9, 0.6, 0.3))),
                            transform: Transform::from_xyz(x, y, 0.1),
                            ..default()
                        },
                        Player {
                            break_cooldown: 0.,
                            attack_cooldown: 0.,
                            place_cooldown: 0.,
                        },
                        CollisionBox {
                            width: 0.77,
                            height: 0.77,
                        },
                        PhysicsBody {
                            vel: Vec2::new(vel_x, vel_y),
                            acc: Vec2::new(acc_x, acc_y),
                        },
                    ));
                }
                "slot" => {
                    // let block_id = parts[1].parse::<usize>().unwrap();
                    let count = parts[2].parse::<usize>().unwrap();

                    match get_block_type_from_string_rep(parts[1], &parts[3..]) {
                        Some(block_type) => {
                            insert_block_to_inventory(&mut inventory, block_type, count);
                        }
                        None => {
                            inventory.slots.push(None);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}

pub fn despawn_players(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    mut inventory: ResMut<PlayerInventory>,
) {
    inventory.blocks.clear();
    inventory.slots.clear();
    inventory.selected_slot = 0;
    for player in &players {
        commands.entity(player).despawn();
    }
}

pub fn load_close_chunks(
    mut commands: Commands,
    mut chunks_to_save: ResMut<ChunksToSave>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut block_map: ResMut<Map>,
    player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let player_translation = player.get_single().unwrap().translation;
    let player_pos = (
        (player_translation.x / 16.).floor() as i32,
        (player_translation.y / 16.).floor() as i32,
    );

    let render_radius = 1;

    for i in -render_radius..(render_radius + 1) {
        for j in -render_radius..(render_radius + 1) {
            load_chunk(
                &mut commands,
                &mut chunks_to_save,
                &mut meshes,
                &mut materials,
                &mut block_map,
                &asset_server,
                (player_pos.0 + i, player_pos.1 + j),
            );
        }
    }
}

pub fn load_chunk(
    commands: &mut Commands,
    chunks_to_save: &mut ResMut<ChunksToSave>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    block_map: &mut ResMut<Map>,
    asset_server: &Res<AssetServer>,
    pos: (i32, i32),
) {
    if chunks_to_save.chunks_loaded.contains(&pos) {
        return;
    }
    chunks_to_save.chunks_loaded.insert(pos);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.7, 0.3),
                custom_size: Some(Vec2::new(16., 16.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (pos.0 * 16) as f32 + 8.,
                (pos.1 * 16) as f32 + 8.,
                -0.1,
            )),
            ..default()
        },
        ChunkBackground {},
    ));
    let mut entities = String::new();
    if let Some(text) = chunks_to_save.chunks.get(&pos) {
        entities.push_str(&text);
        chunks_to_save.chunks.remove(&pos);
    } else if let Ok(text) = fs::read_to_string(format!("./assets/chunk_{}_{}.txt", pos.0, pos.1)) {
        entities.push_str(&text);
    } else {
        // generate the chunk
        generate_trees(commands, meshes, materials, pos);
        generate_rocks(commands, meshes, materials, asset_server, pos);
        println!("wtf");
        return;
    }
    let lines = entities.split('\n').collect::<Vec<&str>>();
    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        match parts[0] {
            "tree" => {
                spawn_tree_from_string_rep(commands, meshes, materials, parts);
            }
            "rock" => {
                spawn_rock_from_string_rep(commands, meshes, materials, parts);
            }
            "block" => {
                let x = parts[2].parse::<i32>().unwrap();
                let y = parts[3].parse::<i32>().unwrap();
                spawn_block(
                    commands,
                    block_map,
                    (x, y),
                    get_block_type_from_string_rep(parts[1], &parts[4..]).unwrap(),
                )
            }
            _ => {}
        }
    }
}

pub fn unload_far_chunk_backgrounds(
    mut commands: Commands,
    backgrounds: Query<(&Transform, Entity), With<ChunkBackground>>,
    mut chunks_to_save: ResMut<ChunksToSave>,
    player: Query<&Transform, With<Player>>,
) {
    let player_translation = player.get_single().unwrap().translation;
    for (background, entity) in &backgrounds {
        let pos = (
            (background.translation.x / 16.).floor() as i32,
            (background.translation.y / 16.).floor() as i32,
        );
        let player_pos = (
            (player_translation.x / 16.).floor() as i32,
            (player_translation.y / 16.).floor() as i32,
        );

        if (pos.0 - player_pos.0).abs() <= 1 && (pos.1 - player_pos.1).abs() <= 1 {
            continue;
        }
        commands.entity(entity).despawn();
        chunks_to_save.chunks_loaded.remove(&pos);
    }
}

pub fn unload_all_chunk_backgrounds_and_clear_chunks_to_save(
    mut commands: Commands,
    backgrounds: Query<Entity, With<ChunkBackground>>,
    mut chunks_to_save: ResMut<ChunksToSave>,
) {
    for entity in &backgrounds {
        commands.entity(entity).despawn();
    }
    chunks_to_save.chunks_loaded.clear();
}

pub fn save_chunks_to_file(mut chunks_to_save: ResMut<ChunksToSave>) {
    for (pos, entities) in chunks_to_save.chunks.iter() {
        if let Ok(_) = fs::write(format!("./assets/chunk_{}_{}.txt", pos.0, pos.1), entities) {
            println!("saved chunk successfully");
        } else {
            println!("error saving chunks");
        }
    }
    chunks_to_save.chunks.clear();
}

pub fn get_block_rep_from_string_rep_and_pos(
    string_rep: (String, String),
    block_pos: (i32, i32),
) -> String {
    let mut block_string = String::new();
    block_string.push_str("block ");
    block_string.push_str(&(string_rep.0).as_str());
    block_string.push(' ');
    block_string.push_str(block_pos.0.to_string().as_str());
    block_string.push(' ');
    block_string.push_str(block_pos.1.to_string().as_str());
    block_string.push(' ');
    block_string.push_str(&(string_rep.1).as_str());
    block_string.push('\n');
    block_string
}

pub fn unload_far_blocks(
    mut commands: Commands,
    blocks: Query<(&Transform, Entity), With<BlockEntity>>,
    mut block_map: ResMut<Map>,
    mut chunks_to_save: ResMut<ChunksToSave>,
    player: Query<&Transform, With<Player>>,
) {
    let player = player.get_single().unwrap().translation;
    for (block_transform, entity) in &blocks {
        let block_pos = (
            block_transform.translation.x as i32,
            block_transform.translation.y as i32,
        );
        let pos = (
            (block_transform.translation.x / 16.).floor() as i32,
            (block_transform.translation.y / 16.).floor() as i32,
        );
        if (pos.0 - (player.x / 16.).floor() as i32).abs() <= 1
            && (pos.1 - (player.y / 16.).floor() as i32).abs() <= 1
        {
            continue;
        }
        let string_rep =
            get_string_rep_from_block_type(Some(block_map.blocks.get(&block_pos).unwrap().1));
        let block_string = get_block_rep_from_string_rep_and_pos(string_rep, block_pos);
        // let mut block_string = String::new();
        // block_string.push_str("block ");
        // block_string.push_str(&(string_rep.0).as_str());
        // block_string.push(' ');
        // block_string.push_str(block_pos.0.to_string().as_str());
        // block_string.push(' ');
        // block_string.push_str(block_pos.1.to_string().as_str());
        // block_string.push(' ');
        // block_string.push_str(&(string_rep.1).as_str());
        // block_string.push('\n');

        if let Some(block_str) = chunks_to_save.chunks.get_mut(&pos) {
            block_str.push_str(block_string.as_str());
        } else {
            chunks_to_save.chunks.insert(pos, block_string);
        }

        commands.entity(entity).despawn();
        block_map.blocks.remove(&block_pos);
    }
}

pub fn save_all_blocks(
    mut commands: Commands,
    blocks: Query<(&Transform, Entity), With<BlockEntity>>,
    mut block_map: ResMut<Map>,
    mut chunks_to_save: ResMut<ChunksToSave>,
) {
    for (block_transform, entity) in &blocks {
        let block_pos = (
            block_transform.translation.x.round() as i32,
            block_transform.translation.y.round() as i32,
        );
        let pos = (
            (block_transform.translation.x / 16.).floor() as i32,
            (block_transform.translation.y / 16.).floor() as i32,
        );
        let string_rep =
            get_string_rep_from_block_type(Some(block_map.blocks.get(&block_pos).unwrap().1));
        let block_string = get_block_rep_from_string_rep_and_pos(string_rep, block_pos);
        // let mut block_string = String::new();
        // let string_rep =
        //     get_string_rep_from_block_type(Some(block_map.blocks.get(&block_pos).unwrap().1));
        // block_string.push_str("block ");
        // block_string.push_str(&(string_rep.0).as_str());
        // block_string.push(' ');
        // block_string.push_str(block_pos.0.to_string().as_str());
        // block_string.push(' ');
        // block_string.push_str(block_pos.1.to_string().as_str());
        // block_string.push(' ');
        // block_string.push_str(&(string_rep.1).as_str());
        // block_string.push('\n');

        if let Some(block_str) = chunks_to_save.chunks.get_mut(&pos) {
            block_str.push_str(block_string.as_str());
        } else {
            chunks_to_save.chunks.insert(pos, block_string);
        }

        commands.entity(entity).despawn();
    }
    block_map.blocks.clear();
}

pub fn unload_far_collectibles(
    mut commands: Commands,
    rocks: Query<(&Collectible, Entity), With<Rock>>,
    trees: Query<(&Collectible, Entity), With<Tree>>,
    mut chunks_to_save: ResMut<ChunksToSave>,
    player: Query<&Transform, With<Player>>,
) {
    let player_translation = player.get_single().unwrap().translation;
    let player_pos = (
        (player_translation.x / 16.).floor() as i32,
        (player_translation.y / 16.).floor() as i32,
    );
    let mut pos: (i32, i32) = (0, 0);

    let mut do_stuff = |collectible: &Collectible, entity: Entity, rep_string: String| {
        pos = (
            (collectible.pos.x / 16.).floor() as i32,
            (collectible.pos.y / 16.).floor() as i32,
        );

        if (pos.0 - player_pos.0).abs() <= 1 && (pos.1 - player_pos.1).abs() <= 1 {
            return;
        }
        if let Some(chunk_str) = chunks_to_save.chunks.get_mut(&pos) {
            chunk_str.push_str(rep_string.as_str());
        } else {
            chunks_to_save.chunks.insert(pos, rep_string);
        }

        commands.entity(entity).despawn();
    };

    for (collectible, entity) in &rocks {
        let rocks_string = make_string_rep_of_rock(collectible);
        do_stuff(collectible, entity, rocks_string);
    }

    for (collectible, entity) in &trees {
        let trees_string = make_string_rep_of_tree(collectible);
        do_stuff(collectible, entity, trees_string);
    }
}

pub fn unload_all_collectibles(
    rocks: Query<(&Collectible, Entity), With<Rock>>,
    trees: Query<(&Collectible, Entity), With<Tree>>,
    mut chunks_to_save: ResMut<ChunksToSave>,
    mut commands: Commands,
) {
    let mut do_stuff = |collectible: &Collectible, entity: Entity, rep_string: String| {
        let pos = (
            (collectible.pos.x / 16.).floor() as i32,
            (collectible.pos.y / 16.).floor() as i32,
        );

        if let Some(chunk_str) = chunks_to_save.chunks.get_mut(&pos) {
            chunk_str.push_str(rep_string.as_str());
        } else {
            chunks_to_save.chunks.insert(pos, rep_string);
        }

        commands.entity(entity).despawn();
    };
    for (collectible, entity) in &rocks {
        let rocks_string = make_string_rep_of_rock(collectible);
        do_stuff(collectible, entity, rocks_string);
    }
    for (collectible, entity) in &trees {
        let trees_string = make_string_rep_of_tree(collectible);
        do_stuff(collectible, entity, trees_string);
    }
}
