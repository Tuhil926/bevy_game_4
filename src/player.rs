use std::collections::HashMap;

use bevy::prelude::*;

use crate::{BlockType, ItemType, PhysicsBody, PLAYER_ACCELERATION};

#[derive(Component)]
pub struct Player {
    pub break_cooldown: f32,
    pub attack_cooldown: f32,
    pub place_cooldown: f32,
}

#[derive(Clone, Copy)]
pub struct InventorySlot {
    pub item_type: ItemType,
    pub count: usize,
}

#[derive(Resource)]
pub struct PlayerInventory {
    pub items: HashMap<ItemType, usize>,
    pub selected_slot: usize,
    pub slots: Vec<Option<InventorySlot>>,
}

pub fn change_player_selected_slot(
    input: Res<Input<KeyCode>>,
    mut inventory: ResMut<PlayerInventory>,
) {
    if input.just_pressed(KeyCode::Key1) {
        inventory.selected_slot = 0;
    } else if input.just_pressed(KeyCode::Key2) {
        inventory.selected_slot = 1;
    } else if input.just_pressed(KeyCode::Key3) {
        inventory.selected_slot = 2;
    } else if input.just_pressed(KeyCode::Key4) {
        inventory.selected_slot = 3;
    } else if input.just_pressed(KeyCode::Key5) {
        inventory.selected_slot = 4;
    } else if input.just_pressed(KeyCode::Key6) {
        inventory.selected_slot = 5;
    } else if input.just_pressed(KeyCode::Key7) {
        inventory.selected_slot = 6;
    } else if input.just_pressed(KeyCode::Key8) {
        inventory.selected_slot = 7;
    } else if input.just_pressed(KeyCode::Key9) {
        inventory.selected_slot = 8;
    }
}

pub fn move_player(input: Res<Input<KeyCode>>, mut players: Query<&mut PhysicsBody, With<Player>>) {
    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::A) {
        direction.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        direction.x += 1.;
    }
    if input.pressed(KeyCode::S) {
        direction.y -= 1.;
    }
    if input.pressed(KeyCode::W) {
        direction.y += 1.;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }
    for mut player in &mut players {
        player.acc = direction * PLAYER_ACCELERATION;
    }
}
