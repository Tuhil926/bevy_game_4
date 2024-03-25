use bevy::{
    prelude::*, render::camera::ScalingMode, time::common_conditions::on_timer,
    window::PrimaryWindow,
};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};
mod chunk_loader_unloader;
mod inventory_ui;
mod player;
mod rock;
mod tree;

use chunk_loader_unloader::*;
use inventory_ui::*;
use player::*;
use rock::*;
use tree::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(Component)]
pub struct PhysicsBody {
    vel: Vec2,
    acc: Vec2,
}

#[derive(Component)]
pub struct CollisionBox {
    width: f32,
    height: f32,
}

#[derive(Component)]
pub struct StaticCollisionCircle {
    radius: f32,
}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum BlockType {
    Wood,
    Stone(i32),
}

#[derive(Component)]
pub struct BlockEntity {
    // pos: (i32, i32),
    // block_type: BlockType,
}
#[derive(Component)]
pub struct Collectible {
    pos: Vec2,
    gather_radius: f32,
    block_type: BlockType,
    uses: usize,
}

const PLAYER_ACCELERATION: f32 = 80.;
const DRAG: f32 = 100.;
const MAP_SIZE: f32 = 2000.;

#[derive(Resource)]
pub struct Map {
    blocks: HashMap<(i32, i32), (Entity, BlockType)>,
}

#[derive(Resource)]
struct MousePosInWorld {
    pos: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InventoryUI)
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.15)))
        .insert_resource(ChunksToSave {
            chunks: HashMap::new(),
            chunks_loaded: HashSet::new(),
        })
        .insert_resource(Map {
            blocks: HashMap::new(),
        })
        .insert_resource(PlayerInventory {
            blocks: HashMap::new(),
            selected_slot: 0,
            slots: vec![],
        })
        .insert_resource(MousePosInWorld { pos: Vec2::ZERO })
        .add_systems(Startup, setup)
        // .add_systems(Update, print_num_entites)
        .add_systems(
            Update,
            (
                move_player,
                entity_collide_static_circle.after(move_player),
                update_physics_body_movement.after(entity_collide_static_circle),
                move_camera,
                block_placer_system,
                entity_collide_block.after(move_player),
                player_gather_collectible,
                calculate_mouse_pos_in_world,
                update_collectibles,
                change_player_selected_slot,
                unload_far_collectibles.run_if(on_timer(Duration::from_millis(500))),
                // unload_far_trees.run_if(on_timer(Duration::from_millis(500))),
                // unload_far_rocks.run_if(on_timer(Duration::from_millis(500))),
                unload_far_blocks.run_if(on_timer(Duration::from_millis(500))),
                unload_far_chunk_backgrounds.run_if(on_timer(Duration::from_millis(500))),
                load_close_chunks.run_if(on_timer(Duration::from_millis(100))),
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(OnEnter(AppState::Game), spawn_players)
        .add_systems(
            Update,
            (transition_to_game_state, transition_to_main_menu_state),
        )
        .add_systems(
            OnExit(AppState::Game),
            (
                save_players.before(despawn_players),
                despawn_players,
                unload_all_collectibles,
                save_all_blocks,
                save_chunks_to_file
                    .after(unload_all_collectibles)
                    .after(save_all_blocks),
                unload_all_chunk_backgrounds_and_clear_chunks_to_save.after(save_chunks_to_file),
            ),
            // (
            //     save_players.before(despawn_players),
            //     despawn_players,
            //     // save_all_trees.before(despawn_all_trees),
            //     // save_all_rocks.before(despawn_all_rocks),
            //     unload_all_collectibles,
            //     save_all_blocks.before(unload_all_chunks),
            //     save_chunks_to_file,
            //     // .after(save_all_blocks)
            //     // .after(save_all_trees)
            //     // .after(save_all_rocks)
            //     // .before(unload_all_chunks),
            //     // despawn_all_trees,
            //     // despawn_all_rocks,
            //     unload_all_chunks,
            // ),
        )
        .run();
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if *app_state.get() != AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if *app_state.get() != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            println!("Entered AppState::MainMenu");
        }
    }
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

// fn spawn_map(mut commands: Commands) {
//     commands.spawn((SpriteBundle {
//         sprite: Sprite {
//             color: Color::rgb(0.5, 0.7, 0.3),
//             custom_size: Some(Vec2::new(MAP_SIZE, MAP_SIZE)),
//             ..default()
//         },
//         transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
//         ..default()
//     },));
// }

fn calculate_mouse_pos_in_world(
    camera: Query<&Transform, (With<Camera>, Without<Player>)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_world: ResMut<MousePosInWorld>,
) {
    let window = q_windows.single();
    if let Some(mut mouse_position) = window.cursor_position() {
        let height = window.height();
        let width = window.width();
        let camera_transform = camera.get_single().unwrap();
        mouse_position.x -= width / 2.;
        mouse_position.y -= height / 2.;

        mouse_position.y /= -height / 10.;
        mouse_position.x /= height / 10.;

        mouse_position.x += camera_transform.translation.x;
        mouse_position.y += camera_transform.translation.y;

        mouse_world.pos = mouse_position;
    }
}

fn insert_block_to_inventory(
    inventory: &mut ResMut<PlayerInventory>,
    block_type: BlockType,
    number: usize,
) {
    let count = *(inventory.blocks.get(&block_type).unwrap_or(&0));
    inventory.blocks.insert(block_type.clone(), count + number);

    let mut empty_slot = inventory.slots.len();

    for i in 0..inventory.slots.len() {
        match &inventory.slots[i] {
            Some(slot) => {
                if slot.item_type == block_type {
                    inventory.slots[i] = Some(InventorySlot {
                        item_type: block_type,
                        count: slot.count + number,
                    });

                    return;
                }
            }
            None => {
                if empty_slot == inventory.slots.len() {
                    empty_slot = i;
                }
            }
        }
    }
    if empty_slot == inventory.slots.len() {
        inventory.slots.push(Some(InventorySlot {
            item_type: block_type,
            count: number,
        }));
    } else {
        inventory.slots[empty_slot] = Some(InventorySlot {
            item_type: block_type,
            count: number,
        });
    }
}

fn pop_block_from_current_slot(inventory: &mut ResMut<PlayerInventory>) -> Option<BlockType> {
    let selected_slot = inventory.selected_slot;
    if selected_slot >= inventory.slots.len() {
        return None;
    }
    if let Some(slot) = &inventory.slots[selected_slot].clone() {
        let &count_in_inventory = inventory.blocks.get(&slot.item_type).unwrap_or(&0);
        let count_in_slot = slot.count;

        let block_type = slot.item_type.clone();
        if count_in_inventory > 1 && count_in_slot > 1 {
            inventory.slots[selected_slot] = Some(InventorySlot {
                item_type: block_type.clone(),
                count: count_in_inventory - 1,
            });
            inventory
                .blocks
                .insert(block_type.clone(), count_in_slot - 1);
            return Some(block_type);
        } else {
            inventory.slots[selected_slot] = None;
            inventory.blocks.remove(&block_type);
            return Some(block_type);
        }
    }
    None
}

// fn get_breaking_time(block_type: BlockType) -> f32 {
//     match block_type {
//         BlockType::Wood => 1.,
//         BlockType::Stone => 2.,
//     }
// }

fn block_placer_system(
    mut commands: Commands,
    mut block_map: ResMut<Map>,
    mouse: Res<Input<MouseButton>>,
    mut player_transforms: Query<(&Transform, &mut Player)>,
    mut inventory: ResMut<PlayerInventory>,
    mouse_world: ResMut<MousePosInWorld>,
    time: Res<Time>,
) {
    let (player_transform, mut player) = player_transforms.get_single_mut().unwrap();
    let player_translation = player_transform.translation;
    if mouse.pressed(MouseButton::Left) || mouse.pressed(MouseButton::Right) {
        let pos = (
            (mouse_world.pos.x + 0.5).floor() as i32,
            (mouse_world.pos.y + 0.5).floor() as i32,
        );

        if pos.0.abs() > MAP_SIZE as i32 / 2
            || pos.1.abs() > MAP_SIZE as i32 / 2
            || (player_translation.x.round() as i32 == pos.0
                && player_translation.y.round() as i32 == pos.1)
        {
            return;
        }

        if mouse.pressed(MouseButton::Right) {
            player.break_cooldown = 0.;
            if player.place_cooldown <= 0. {
                player.place_cooldown = 0.;
                if !block_map.blocks.contains_key(&pos) {
                    player.place_cooldown = 0.2;
                    if let Some(block_type) = pop_block_from_current_slot(&mut inventory) {
                        println!("block placed at {}, {}", pos.0, pos.1);
                        spawn_block(&mut commands, &mut block_map, pos, block_type);
                    }
                }
            } else {
                player.place_cooldown -= time.delta_seconds();
            }
        } else {
            player.place_cooldown = 0.;
            if player.break_cooldown >= 0.5 {
                player.break_cooldown = 0.;
                if let Some(block_type) = despawn_block(commands, block_map, pos) {
                    insert_block_to_inventory(&mut inventory, block_type, 1);
                }
            } else if let Some(_) = block_map.blocks.get(&pos) {
                player.break_cooldown += time.delta_seconds();
            } else {
                player.break_cooldown = 0.;
            }
        }
    } else {
        player.place_cooldown = 0.;
        player.break_cooldown = 0.;
    }
}

fn despawn_block(
    mut commands: Commands,
    mut block_map: ResMut<Map>,
    pos: (i32, i32),
) -> Option<BlockType> {
    if let Some((entity, block_type)) = block_map.blocks.remove(&pos) {
        commands.entity(entity).despawn();
        return Some(block_type.clone());
    }
    None
}

fn get_block_color(block_type: BlockType) -> Color {
    match block_type {
        BlockType::Wood => Color::rgb(0.4, 0.2, 0.),
        BlockType::Stone(_) => Color::rgb(0.3, 0.3, 0.3),
    }
}

fn spawn_block(
    commands: &mut Commands,
    block_map: &mut ResMut<Map>,
    pos: (i32, i32),
    block_type: BlockType,
) {
    let id = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: {
                        match block_type {
                            BlockType::Wood => Color::rgb(0.4, 0.2, 0.),
                            BlockType::Stone(_) => Color::rgb(0.3, 0.3, 0.3),
                        }
                    },
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(pos.0 as f32, pos.1 as f32, 0.),
                    ..default()
                },
                ..default()
            },
            BlockEntity {
                // pos: pos,
                // block_type: block_type,
            },
        ))
        .id();

    block_map.blocks.insert(pos, (id, block_type));
}

fn update_physics_body_movement(
    time: Res<Time>,
    mut bodies: Query<(&mut Transform, &mut PhysicsBody)>,
) {
    for (mut transform, mut body) in &mut bodies {
        let acc = body.acc;
        body.vel += acc * time.delta_seconds();
        transform.translation += time.delta_seconds() * body.vel.extend(0.);
        let size = Vec2::splat(MAP_SIZE / 2.).extend(0.);
        if transform.translation.clamp(-size, size) != transform.translation {
            if transform.translation.x != transform.translation.x.clamp(-size.x, size.x) {
                body.vel.x = 0.;
            }
            if transform.translation.y != transform.translation.y.clamp(-size.y, size.y) {
                body.vel.y = 0.;
            }
            transform.translation = transform.translation.clamp(-size, size);
        }

        let vel = body.vel;

        body.vel -= vel * (vel.distance(Vec2::ZERO) + 1.) * DRAG * time.delta_seconds() / 100.;
    }
}

fn entity_collide_block(
    mut bodies: Query<(&mut Transform, &CollisionBox, &mut PhysicsBody)>,
    block_map: ResMut<Map>,
) {
    for (mut transform, coll_box, mut body) in &mut bodies {
        let body_pos = transform.translation;

        let to_check = vec![
            (0, 0),
            (0, -1),
            (-1, 0),
            (0, 1),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        let mut need_to_check_corners = true;

        for i in 0..5 {
            let block_pos = (
                body_pos.x.round() as i32 + to_check[i].0,
                body_pos.y.round() as i32 + to_check[i].1,
            );
            if block_map.blocks.contains_key(&block_pos) {
                let block_entity = block_map.blocks.get(&block_pos).unwrap().clone();
                match block_entity.1 {
                    BlockType::Wood => {}
                    BlockType::Stone(_) => {}
                }
                if (body_pos.x - block_pos.0 as f32).abs() < 0.5 + coll_box.width / 2.
                    && (body_pos.y - block_pos.1 as f32).abs() < 0.5 + coll_box.height / 2.
                {
                    if (body_pos.x - block_pos.0 as f32).abs()
                        > (body_pos.y - block_pos.1 as f32).abs()
                    {
                        if (body_pos.x - block_pos.0 as f32) > 0. {
                            transform.translation.x =
                                block_pos.0 as f32 + 0.5 + coll_box.width / 2.;
                            need_to_check_corners = false;
                        } else {
                            transform.translation.x =
                                block_pos.0 as f32 - 0.5 - coll_box.width / 2.;
                            need_to_check_corners = false;
                        }
                        body.vel.x = 0.;
                    } else {
                        if (body_pos.y - block_pos.1 as f32) > 0. {
                            transform.translation.y =
                                block_pos.1 as f32 + 0.5 + coll_box.height / 2.;
                            need_to_check_corners = false;
                        } else {
                            transform.translation.y =
                                block_pos.1 as f32 - 0.5 - coll_box.height / 2.;
                            need_to_check_corners = false;
                        }
                        body.vel.y = 0.;
                    }
                }
            }
        }
        for i in 5..9 {
            if !need_to_check_corners {
                break;
            }
            let block_pos = (
                body_pos.x.round() as i32 + to_check[i].0,
                body_pos.y.round() as i32 + to_check[i].1,
            );
            if block_map.blocks.contains_key(&block_pos) {
                let block_entity = block_map.blocks.get(&block_pos).unwrap().clone();
                match block_entity.1 {
                    BlockType::Wood => {}
                    _ => {
                        continue;
                    }
                }
                if (body_pos.x - block_pos.0 as f32).abs() < 0.5 + coll_box.width / 2.
                    && (body_pos.y - block_pos.1 as f32).abs() < 0.5 + coll_box.height / 2.
                {
                    if (body_pos.x - block_pos.0 as f32).abs()
                        > (body_pos.y - block_pos.1 as f32).abs()
                    {
                        body.vel.x = 0.;
                        if (body_pos.x - block_pos.0 as f32) > 0. {
                            transform.translation.x =
                                block_pos.0 as f32 + 0.5 + coll_box.width / 2.;
                            break;
                        } else {
                            transform.translation.x =
                                block_pos.0 as f32 - 0.5 - coll_box.width / 2.;
                            break;
                        }
                    } else {
                        body.vel.y = 0.;
                        if (body_pos.y - block_pos.1 as f32) > 0. {
                            transform.translation.y =
                                block_pos.1 as f32 + 0.5 + coll_box.height / 2.;
                            break;
                        } else {
                            transform.translation.y =
                                block_pos.1 as f32 - 0.5 - coll_box.height / 2.;
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn entity_collide_static_circle(
    mut bodies: Query<(&Transform, &mut PhysicsBody)>,
    static_circles: Query<(&Transform, &StaticCollisionCircle)>,
) {
    for (transform, mut body) in &mut bodies {
        for (circle_transform, circle) in &static_circles {
            if transform.translation.distance(circle_transform.translation) < circle.radius {
                body.acc += (transform.translation - circle_transform.translation)
                    .truncate()
                    .normalize()
                    * 400.;

                println!("sdfsdf");
            }
        }
    }
    // println!("lol");
}

fn update_collectibles(mut collectibles: Query<(&mut Transform, &Collectible)>) {
    for (mut transform, collectible) in &mut collectibles {
        if transform.translation.truncate().distance(collectible.pos) > 0.1 {
            let prev = transform.translation;
            transform.translation -= (prev - collectible.pos.extend(0.)) / 5.;
        }
    }
}

fn player_gather_collectible(
    mut commands: Commands,
    mut player: Query<(&Transform, &mut Player)>,
    mouse: Res<Input<MouseButton>>,
    mut inventory: ResMut<PlayerInventory>,
    mut collectibles: Query<
        (&mut Transform, &mut Collectible, Entity),
        (With<StaticCollisionCircle>, Without<Player>),
    >,
    mouse_world: ResMut<MousePosInWorld>,
    time: Res<Time>,
) {
    let (player_transform, mut player) = player.get_single_mut().unwrap();
    if mouse.pressed(MouseButton::Left) {
        if player.attack_cooldown >= 0.5 {
            player.attack_cooldown = 0.;
            let player_direction =
                (mouse_world.pos - player_transform.translation.truncate()).normalize();
            for (mut collectible_transform, mut collectible, entity) in &mut collectibles {
                let collectible_direction =
                    (collectible_transform.translation - player_transform.translation).truncate();
                if collectible_direction.distance(Vec2::ZERO) < collectible.gather_radius
                    && collectible_direction.normalize().dot(player_direction) > 0.1
                {
                    insert_block_to_inventory(&mut inventory, collectible.block_type.clone(), 1);
                    collectible.uses -= 1;
                    if collectible.uses == 0 {
                        commands.entity(entity).despawn();
                    }
                    collectible_transform.translation +=
                        collectible_direction.normalize().extend(0.) / 5.;
                }
            }
        } else {
            player.attack_cooldown += time.delta_seconds();
        }
    } else {
        player.attack_cooldown = 0.;
    }
}

fn move_camera(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_translation = player.get_single().unwrap().translation;
    let mut camera_transform = camera.get_single_mut().unwrap();

    camera_transform.translation.x +=
        (player_translation.x - camera_transform.translation.x) * time.delta_seconds() * 5.;
    camera_transform.translation.y +=
        (player_translation.y - camera_transform.translation.y) * time.delta_seconds() * 5.;
}
