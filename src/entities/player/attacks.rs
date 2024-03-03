use crate::entities::data::{ AttackableFrom, Damage, EntityType, Player, PlayerAttackCoolDown};

use crate::ui::{AttackType, InventorySlot, SelectedSlot};

use bevy::app::{App, Plugin, Update};
use bevy::input::Input;

use bevy::prelude::*;
use bevy_xpbd_2d::components::CollidingEntities;
use crate::entities::event::EntityDamageEvent;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_attack)
            .add_systems(Update, damage_entities)
            .add_event::<PlayerAttackEvent>();
    }
}

#[derive(Component)]
pub struct LongTimeAttack {
    pub damaged_entities: Vec<Entity>,
}

fn damage_entities(
    mut entity_attacks: Query<(&CollidingEntities, &mut LongTimeAttack, &Damage)>,
    attackable_from: Query<&AttackableFrom>,
    mut event_writer: EventWriter<EntityDamageEvent>,
) {
    for (touching_entities, mut entity_attack, damage) in entity_attacks.iter_mut() {
        for touching_entity in &touching_entities.0 {
            let can_be_attacked = attackable_from
                .get(*touching_entity)
                .map(|attackable_from| attackable_from.0.contains(&EntityType::Player))
                .unwrap_or(false);

            if !can_be_attacked || entity_attack.damaged_entities.contains(touching_entity) {
                continue;
            }

            entity_attack.damaged_entities.push(*touching_entity);
            event_writer.send(EntityDamageEvent {
                entity: *touching_entity,
                damage: damage.0,
            })
        }
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
