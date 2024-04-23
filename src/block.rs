use bevy::prelude::*;

use crate::{create_block_update, BlockUpdateQueue, Map};

#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
pub enum BlockType {
    Wood,
    Stone(i32),
    Wire(i32),
    Repeater(i32, i32),
    Inverter(i32, i32),
}

#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
pub enum ItemType {
    Wood,
    Stone(i32),
    Wire(i32),
    Repeater(i32, i32),
    Inverter(i32, i32),
}

#[derive(Component)]
pub struct BlockEntity {
    pub pos: (i32, i32),
    // block_type: BlockType,
}

pub fn get_block_breaking_speed_multiplier(block_type: BlockType) -> f32 {
    match block_type {
        BlockType::Wood => 10.,
        BlockType::Stone(_) => 1.,
        BlockType::Wire(_) => 100.,
        BlockType::Repeater(_, _) => 100.,
        BlockType::Inverter(_, _) => 100.,
    }
}

pub fn despawn_block(
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

pub fn get_block_color(block_type: BlockType) -> Color {
    match block_type {
        BlockType::Wood => Color::rgb(0.4, 0.2, 0.),
        BlockType::Stone(_) => Color::rgb(0.3, 0.3, 0.3),
        BlockType::Wire(power) => Color::rgb(0.4 + 0.004 * power as f32, 0., 0.),
        BlockType::Repeater(power, _) => default(),
        BlockType::Inverter(power, _) => default(),
    }
}

pub fn get_item_color(item_type: ItemType) -> Color {
    match item_type {
        ItemType::Wood => Color::rgb(0.4, 0.2, 0.),
        ItemType::Stone(_) => Color::rgb(0.3, 0.3, 0.3),
        ItemType::Wire(power) => Color::rgb(0.4 + 0.004 * power as f32, 0., 0.),
        ItemType::Repeater(power, _) => default(),
        ItemType::Inverter(power, _) => default(),
    }
}

pub fn get_block_dir(block_type: BlockType) -> i32 {
    match block_type {
        BlockType::Repeater(_, dir) => dir,
        BlockType::Inverter(_, dir) => dir,
        _ => 0,
    }
}

pub fn get_block_texture(block_type: BlockType, asset_server: &Res<AssetServer>) -> Handle<Image> {
    match block_type {
        BlockType::Inverter(power, dir) => {
            if power == 0 {
                asset_server.load("inverter_powered.png")
            } else {
                asset_server.load("inverter_unpowered.png")
            }
        }
        BlockType::Repeater(power, dir) => {
            if power == 0 {
                asset_server.load("repeater_unpowered.png")
            } else {
                asset_server.load("repeater_powered.png")
            }
        }
        _ => default(),
    }
}

pub fn get_item_texture(item_type: ItemType, asset_server: &Res<AssetServer>) -> Handle<Image> {
    match item_type {
        ItemType::Inverter(power, dir) => {
            if power == 0 {
                asset_server.load("inverter_powered.png")
            } else {
                asset_server.load("inverter_unpowered.png")
            }
        }
        ItemType::Repeater(power, dir) => {
            if power == 0 {
                asset_server.load("repeater_unpowered.png")
            } else {
                asset_server.load("repeater_powered.png")
            }
        }
        _ => default(),
    }
}

pub fn spawn_block(
    commands: &mut Commands,
    block_map: &mut ResMut<Map>,
    pos: (i32, i32),
    block_type: BlockType,
    block_update_queue: &mut ResMut<BlockUpdateQueue>,
    asset_server: &Res<AssetServer>,
) {
    let id = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: { get_block_color(block_type) },
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(pos.0 as f32, pos.1 as f32, 0.),
                    ..default()
                }
                .with_rotation(Quat::from_rotation_z(
                    (get_block_dir(block_type) as f32) * -3.1415926535 / 2.,
                )),
                texture: get_block_texture(block_type, asset_server),
                ..default()
            },
            BlockEntity {
                pos: pos,
                // block_type: block_type,
            },
        ))
        .id();

    block_map.blocks.insert(pos, (id, block_type));
    create_block_update(pos, block_update_queue);
}

pub fn get_block_type_from_string_rep(block_type: &str, data: &[&str]) -> Option<BlockType> {
    // dbg!(data);
    match block_type {
        "wood" => Some(BlockType::Wood),
        "stone" => Some(BlockType::Stone(if data.len() > 0 {
            data[0].parse::<i32>().unwrap()
        } else {
            0
        })),
        "wire" => Some(BlockType::Wire(if data.len() > 0 {
            data[0].parse::<i32>().unwrap()
        } else {
            0
        })),
        "repeater" => Some(BlockType::Repeater(
            if data.len() > 0 {
                data[0].parse::<i32>().unwrap()
            } else {
                0
            },
            if data.len() > 1 {
                data[1].parse::<i32>().unwrap()
            } else {
                2
            },
        )),
        "inverter" => Some(BlockType::Inverter(
            if data.len() > 0 {
                data[0].parse::<i32>().unwrap()
            } else {
                1
            },
            if data.len() > 1 {
                data[1].parse::<i32>().unwrap()
            } else {
                2
            },
        )),
        _ => None,
    }
}

pub fn get_string_rep_from_block_type(block_type: Option<BlockType>) -> (String, String) {
    match block_type {
        Some(block) => match block {
            BlockType::Wood => (String::from("wood"), String::new()),
            BlockType::Stone(number) => (String::from("stone"), format!("{}", number)),
            BlockType::Wire(power) => (String::from("wire"), format!("{}", power)),
            BlockType::Repeater(power, dir) => {
                (String::from("repeater"), format!("{} {}", power, dir))
            }
            BlockType::Inverter(power, dir) => {
                (String::from("inverter"), format!("{} {}", power, dir))
            }
        },
        None => (String::from("nothing"), String::new()),
    }
}

pub fn get_item_type_from_string_rep(item_type: &str, data: &[&str]) -> Option<ItemType> {
    // dbg!(data);
    match item_type {
        "wood" => Some(ItemType::Wood),
        "stone" => Some(ItemType::Stone(if data.len() > 0 {
            data[0].parse::<i32>().unwrap()
        } else {
            0
        })),
        "wire" => Some(ItemType::Wire(if data.len() > 0 {
            data[0].parse::<i32>().unwrap()
        } else {
            0
        })),
        "repeater" => Some(ItemType::Repeater(
            if data.len() > 0 {
                data[0].parse::<i32>().unwrap()
            } else {
                0
            },
            if data.len() > 1 {
                data[1].parse::<i32>().unwrap()
            } else {
                2
            },
        )),
        "inverter" => Some(ItemType::Inverter(
            if data.len() > 0 {
                data[0].parse::<i32>().unwrap()
            } else {
                1
            },
            if data.len() > 1 {
                data[1].parse::<i32>().unwrap()
            } else {
                2
            },
        )),
        _ => None,
    }
}

pub fn get_string_rep_from_item_type(item_type: Option<ItemType>) -> (String, String) {
    match item_type {
        Some(block) => match block {
            ItemType::Wood => (String::from("wood"), String::new()),
            ItemType::Stone(number) => (String::from("stone"), format!("{}", number)),
            ItemType::Wire(power) => (String::from("wire"), format!("{}", power)),
            ItemType::Repeater(power, dir) => {
                (String::from("repeater"), format!("{} {}", power, dir))
            }
            ItemType::Inverter(power, dir) => {
                (String::from("inverter"), format!("{} {}", power, dir))
            }
        },
        None => (String::from("nothing"), String::new()),
    }
}

pub fn get_block_rep_from_string_rep_and_pos(
    string_rep: (String, String),
    block_pos: (i32, i32),
) -> String {
    format!(
        "block {} {} {} {}\n",
        string_rep.0, block_pos.0, block_pos.1, string_rep.1
    )
}
pub fn remove_block_type_data_for_inventory(block_type: BlockType) -> ItemType {
    match block_type {
        BlockType::Stone(_) => ItemType::Stone(0),
        BlockType::Wire(_) => ItemType::Wire(0),
        BlockType::Repeater(_, _) => ItemType::Repeater(0, 2),
        BlockType::Inverter(_, _) => ItemType::Inverter(1, 2),
        BlockType::Wood => ItemType::Wood,
    }
}

pub fn get_block_type_after_popped_from_inventory(item_type: ItemType, dir: i32) -> BlockType {
    match item_type {
        ItemType::Repeater(power, _) => BlockType::Repeater(power, (dir + 2) % 4),
        ItemType::Inverter(power, _) => BlockType::Inverter(power, (dir + 2) % 4),
        ItemType::Stone(a) => BlockType::Stone(a),
        ItemType::Wire(power) => BlockType::Wire(power),
        ItemType::Wood => BlockType::Wood,
    }
}
