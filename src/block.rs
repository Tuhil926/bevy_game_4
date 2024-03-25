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
        BlockType::Repeater(power, _) => Color::rgb(
            0.5 + 0.5 * power as f32,
            0.5 + 0.5 * power as f32,
            0.5 + 0.5 * power as f32,
        ),
        BlockType::Inverter(power, _) => {
            Color::rgb(0., 0.5 + 0.5 * power as f32, 0.5 + 0.5 * (1 - power) as f32)
        }
    }
}

pub fn spawn_block(
    commands: &mut Commands,
    block_map: &mut ResMut<Map>,
    pos: (i32, i32),
    block_type: BlockType,
    block_update_queue: &mut ResMut<BlockUpdateQueue>,
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
                },
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
    dbg!(data);
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

pub fn get_block_rep_from_string_rep_and_pos(
    string_rep: (String, String),
    block_pos: (i32, i32),
) -> String {
    format!(
        "block {} {} {} {}\n",
        string_rep.0, block_pos.0, block_pos.1, string_rep.1
    )
}
pub fn remove_block_type_data_for_inventory(block_type: BlockType) -> BlockType {
    match block_type {
        BlockType::Stone(_) => BlockType::Stone(0),
        BlockType::Wire(_) => BlockType::Wire(0),
        BlockType::Repeater(_, _) => BlockType::Repeater(0, 2),
        BlockType::Inverter(_, _) => BlockType::Inverter(1, 2),
        _ => block_type,
    }
}

pub fn get_block_type_after_popped_from_inventory(block_type: BlockType, dir: i32) -> BlockType {
    match block_type {
        BlockType::Repeater(power, _) => BlockType::Repeater(power, (dir + 2) % 4),
        BlockType::Inverter(power, _) => BlockType::Inverter(power, (dir + 2) % 4),
        BlockType::Stone(a) => BlockType::Stone(a),
        BlockType::Wire(power) => BlockType::Wire(power),
        BlockType::Wood => BlockType::Wood,
    }
}
