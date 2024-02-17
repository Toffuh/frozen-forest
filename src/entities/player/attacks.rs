use crate::entities::data::{
    AttackableFrom, Damage, DespawnTimer, EntityType, Player, PlayerAttackCoolDown, PLAYER_RADIUS,
};
use crate::entities::event::EntityDamageEvent;
use crate::entities::player::{handle_keyboard_input, move_player, player_setup, PlayerPlugin};
use crate::ui::{AttackType, InventorySlot, SelectedSlot};
use crate::PhysicsLayers;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::input::Input;
use bevy::math::{vec2, Vec2, Vec2Swizzles, Vec3Swizzles};
use bevy::prelude::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_attack)
            .add_event::<PlayerAttackEvent>();
    }
}

#[derive(Event, PartialEq)]
pub struct PlayerAttackEvent(pub AttackType);

fn player_attack(
    mouse_button_input: Res<Input<MouseButton>>,
    mut player_query: Query<&mut PlayerAttackCoolDown, With<Player>>,
    time: Res<Time>,
    mut attack_event: EventWriter<PlayerAttackEvent>,
    selected_inventory_slot: Res<SelectedSlot>,
    inventory: Query<&InventorySlot>,
) {
    let mut attack_timer = player_query.single_mut();

    attack_timer.0.tick(time.delta());

    if !attack_timer.0.finished() || !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let attack_type = &inventory
        .iter()
        .find(|slot| slot.index == selected_inventory_slot.index)
        .expect(&*format!(
            "Invalid slot selected {}",
            selected_inventory_slot.index
        ))
        .attack;

    if let Some(attack_type) = attack_type {
        attack_event.send(PlayerAttackEvent(*attack_type));
        attack_timer.0.reset();
    }
}
