use bevy::prelude::*;

use crate::{get_block_color, get_block_texture, AppState, InventorySlot, PlayerInventory};

#[derive(Component)]
pub struct InventoryUI;

#[derive(Component, Clone, Copy)]
pub struct InventoryUISlot {
    pub index: usize,
    pub item_slot: Option<InventorySlot>,
}

impl Plugin for InventoryUI {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_inventory_ui);
        app.add_systems(OnExit(AppState::Game), despawn_inventory_ui);
        app.add_systems(
            Update,
            (update_inventory_ui).run_if(in_state(AppState::Game)),
        );
    }
}

pub fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_inventory_ui(&mut commands, &asset_server);
}

pub fn despawn_inventory_ui(
    inventory_ui: Query<Entity, With<InventoryUI>>,
    mut commands: Commands,
) {
    for entity in inventory_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_inventory_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let inventory_ui_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(15.),
                    top: Val::Percent(85.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                // background_color: Color::rgba(0., 0., 0., 0.5).into(),
                ..default()
            },
            InventoryUI {},
        ))
        .with_children(|parent| {
            for i in 0..9 {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(64.),
                            width: Val::Px(64.),
                            margin: UiRect::horizontal(Val::Px(10.)),
                            ..default()
                        },
                        transform: Transform::default()
                            .with_rotation(Quat::from_rotation_z(3.1415926535)),
                        background_color: Color::RED.into(),
                        ..default()
                    },
                    InventoryUISlot {
                        index: i,
                        item_slot: None,
                    },
                ));
            }
        })
        .id();
    inventory_ui_entity
}

fn update_inventory_ui(
    mut inventory_ui_slots: Query<(
        &mut BackgroundColor,
        &mut Style,
        &mut UiImage,
        &InventoryUISlot,
    )>,
    inventory: Res<PlayerInventory>,
    asset_server: Res<AssetServer>,
) {
    for (mut color, mut style, mut img, &inventory_ui_slot) in &mut inventory_ui_slots {
        if let Some(Some(slot)) = inventory.slots.get(inventory_ui_slot.index) {
            color.0 = get_block_color(slot.item_type);
            *img = get_block_texture(slot.item_type, &asset_server).into();
        } else {
            color.0 = Color::rgba(0.0, 0.0, 0.0, 0.3);
            *img = default();
        }

        if inventory.selected_slot == inventory_ui_slot.index {
            style.width = Val::Px(80.);
            style.height = Val::Px(80.);
        } else {
            style.width = Val::Px(64.);
            style.height = Val::Px(64.);
        }
    }
}
