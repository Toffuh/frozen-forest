use crate::entities::data::{
    AttackTimer, AttackableFrom, Damage, DamageCoolDown, DespawnTimer, EntityType, Health, Mob,
    Player,
};
use crate::entities::event::{EntityDamageEvent, EntityDeathEvent};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    Color, Commands, Entity, EventReader, EventWriter, Or, Query, Res, Sprite, Time, With, Without,
};
use bevy_xpbd_2d::components::CollidingEntities;
use iter_tools::Itertools;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (deal_damage_on_collision, deal_damage, remove_dead_entities),
        )
        .add_systems(
            Update,
            (
                tick_damage_cool_down,
                remove_damage_cool_down,
                color_mob_on_damage,
                despawn,
            ),
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
    mut commands: Commands,
    mut event_writer: EventWriter<EntityDeathEvent>,
    mut event_reader: EventReader<EntityDamageEvent>,
    mut health: Query<&mut Health, Without<DamageCoolDown>>,
) {
    for event in event_reader
        .read()
        .dedup_by(|event_a, event_b| event_a.entity == event_b.entity)
    {
        let Ok(mut health) = health.get_mut(event.entity) else {
            return;
        };

        if health.0 - event.damage <= 0. {
            event_writer.send(EntityDeathEvent(event.entity));
            return;
        }

        commands
            .entity(event.entity)
            .insert(DamageCoolDown::new(0.3));
        health.0 -= event.damage;
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

pub fn tick_damage_cool_down(time: Res<Time>, mut entities: Query<&mut DamageCoolDown>) {
    for mut cool_down in entities.iter_mut() {
        cool_down.0.tick(time.delta());
    }
}

pub fn remove_damage_cool_down(mut commands: Commands, entities: Query<(Entity, &DamageCoolDown)>) {
    for (entity, _) in entities
        .iter()
        .filter(|(_, cool_down)| cool_down.0.finished())
    {
        commands.entity(entity).remove::<DamageCoolDown>();
    }
}

pub fn color_mob_on_damage(
    mut mobs: Query<(Option<&DamageCoolDown>, &mut Sprite), Or<(With<Player>, With<Mob>)>>,
) {
    for (cool_down, mut sprite) in mobs.iter_mut() {
        if cool_down.is_some() {
            sprite.color = Color::rgb(1., 0.75, 0.25)
        } else {
            sprite.color = Color::rgb(0.25, 0.75, 0.25)
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut mobs: Query<(Entity, &mut DespawnTimer)>,
) {
    for (entity, mut timer) in mobs.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            commands.entity(entity).despawn()
        }
    }
}
