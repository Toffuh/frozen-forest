use crate::entities::data::{Health, Player, MAX_PLAYER_HEALTH};
use bevy::app::{App, Startup};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_health_bar)
            .add_systems(Update, update_health_bar)
            .add_systems(Startup, setup)
            .add_systems(Update, set_border_color)
            .add_systems(Update, select_inventory_slot);
    }
}

#[derive(Component)]
pub struct HealthBar;

pub fn setup_health_bar(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Px(200.),
            height: Val::Px(20.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        background_color: Color::rgb(0., 0., 0.).into(),
        ..default()
    };

    let bar = (
        HealthBar,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            background_color: Color::rgb(0.9, 0.25, 0.35).into(),
            ..default()
        },
    );

    commands.spawn(container).with_children(|parent| {
        parent.spawn(bar);
    });
}

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
) {
    let Ok(health) = player_query.get_single() else {
        warn!("could not find a single Player");
        return;
    };

    let Ok(mut heal_bar) = health_bar_query.get_single_mut() else {
        warn!("could not find a single HealthBar");
        return;
    };

    heal_bar.width = Val::Percent((health.0 / MAX_PLAYER_HEALTH) as f32 * 100.);
}

#[derive(PartialEq, Component, Debug)]
pub enum SpellType {
    Fireball,
    None,
}

#[derive(Component)]
pub struct InventorySlot {
    pub index: usize,
    pub spell: SpellType,
}

#[derive(Resource)]
pub struct SelectedSlot {
    pub index: usize,
}

fn setup(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            ..Default::default()
        },
        ..Default::default()
    };

    let inventory_slot = NodeBundle {
        style: Style {
            width: Val::Px(50.),
            height: Val::Px(50.),
            border: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn(container).with_children(|parent| {
        parent
            .spawn(inventory_slot.clone())
            .insert(InventorySlot { index: 1, spell: SpellType::Fireball });

        for i in 1..5 {
            parent
                .spawn(inventory_slot.clone())
                .insert(InventorySlot { index: i + 1, spell: SpellType::None });
        }
    });

    commands.insert_resource(SelectedSlot { index: 1 });
}

fn select_inventory_slot(
    keyboard_input: Res<Input<KeyCode>>,
    mut selected_slot: ResMut<SelectedSlot>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        selected_slot.index = 1;
    }
    if keyboard_input.just_pressed(KeyCode::Key2) {
        selected_slot.index = 2;
    }
    if keyboard_input.just_pressed(KeyCode::Key3) {
        selected_slot.index = 3;
    }
    if keyboard_input.just_pressed(KeyCode::Key4) {
        selected_slot.index = 4;
    }
    if keyboard_input.just_pressed(KeyCode::Key5) {
        selected_slot.index = 5;
    }
}

fn set_border_color(
    mut query: Query<(&InventorySlot, &mut BorderColor)>,
    selected_slot: Res<SelectedSlot>,
) {
    for (inventory_slot, mut background_color) in query.iter_mut() {
        if inventory_slot.index == selected_slot.index {
            background_color.0 = Color::RED
        } else {
            background_color.0 = Color::BLACK;
        }
    }
}
