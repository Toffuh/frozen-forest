use crate::entities::data::{AttackTimer, AttackableFrom, Damage, EntityType, Health, Player};
use crate::entities::event::{EntityDamageEvent, EntityDeathEvent};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, Time, With, Without};
use bevy_xpbd_2d::components::CollidingEntities;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (deal_damage_on_collision, deal_damage, remove_dead_entities),
        );
    }
}

pub fn deal_damage_on_collision(
    mut attacked_entities: Query<(&CollidingEntities, Entity, &AttackableFrom), With<Health>>,
    time: Res<Time>,
    mut event_writer: EventWriter<EntityDamageEvent>,
    mut attacking_entities: Query<(&mut AttackTimer, &Damage, &EntityType), Without<Player>>,
) {
    for (attacking, damageable_entity, attackable_from) in attacked_entities.iter_mut() {
        for attacking_entity in &attacking.0 {
            let Ok((mut timer, damage, entity_type)) =
                attacking_entities.get_mut(*attacking_entity)
            else {
                continue;
            };

            if !attackable_from.0.contains(entity_type) {
                continue;
            }

            timer.0.tick(time.delta());

            if !timer.0.finished() {
                continue;
            }

            event_writer.send(EntityDamageEvent {
                entity: damageable_entity,
                damage: damage.0,
            });
        }
    }
}

pub fn deal_damage(
    mut event_writer: EventWriter<EntityDeathEvent>,
    mut event_reader: EventReader<EntityDamageEvent>,
    mut health: Query<&mut Health>,
) {
    for entity_damage_event in event_reader.read() {
        let health = &mut health.get_mut(entity_damage_event.entity).unwrap().0;

        if *health - entity_damage_event.damage <= 0. {
            event_writer.send(EntityDeathEvent(entity_damage_event.entity));
            continue;
        }

        *health -= entity_damage_event.damage;
    }
}

pub fn remove_dead_entities(
    mut event_reader: EventReader<EntityDeathEvent>,
    mut commands: Commands,
) {
    for dead_entity in event_reader.read() {
        commands.entity(dead_entity.0).despawn();
    }
}
