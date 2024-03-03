use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Entity, EventWriter, Query};
use bevy_xpbd_2d::prelude::CollidingEntities;
use crate::entities::data::{AttackableFrom, Damage, EntityType};
use crate::entities::event::{EntityDamageEvent};

pub struct LongTimeAttackPlugin;

impl Plugin for LongTimeAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, damage_entities);
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
            });
        }
    }
}