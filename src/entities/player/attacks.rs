use crate::entities::data::{ AttackableFrom, Damage, EntityType, Player, PlayerAttackCoolDown};

use crate::ui::{AttackType, InventorySlot, SelectedSlot};

use bevy::app::{App, Plugin, Update};

use bevy::prelude::*;
use bevy_xpbd_2d::components::CollidingEntities;
use crate::entities::event::EntityDamageEvent;

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
    mouse_button_input: Res<ButtonInput<MouseButton>>,
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

    if let Some(slot) = &inventory
        .iter()
        .find(|slot| slot.index == selected_inventory_slot.index)
    {
        if let Some(attack_type) = slot.attack {
            attack_event.send(PlayerAttackEvent(attack_type));
            attack_timer.0.reset();
        }
    } else {
        error!("Invalid slot selected {}", selected_inventory_slot.index)
    }
}
