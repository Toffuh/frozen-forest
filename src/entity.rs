use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    Commands, Component, Entity, Event, EventReader, EventWriter, Query, Res, Time, Timer,
    TimerMode, With,
};
use bevy_xpbd_2d::components::CollidingEntities;

static HIT_COOLDOWN: f32 = 1.;

#[derive(Component)]
pub struct Damage(pub f64);

#[derive(Component)]
pub struct Health(pub f64);

#[derive(PartialEq)]
pub enum Layer {
    Player,
    Mob,
    Wall,
}

#[derive(Component)]
pub struct AttackableFrom(pub Vec<Layer>);

#[derive(Component)]
pub struct LayerType(pub Layer);

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (deal_damage_on_collision, deal_damage, remove_dead_entities),
        )
        .add_event::<EntityDamageEvent>()
        .add_event::<EntityDeathEvent>();
    }
}

#[derive(Component)]
pub struct DamageTimer(pub(crate) Timer);
impl Default for DamageTimer {
    fn default() -> DamageTimer {
        DamageTimer(Timer::from_seconds(HIT_COOLDOWN, TimerMode::Repeating))
    }
}

#[derive(Event)]
pub struct EntityDamageEvent {
    entity: Entity,
    damage: f64,
}

pub fn deal_damage_on_collision(
    mut damageable_entities: Query<(&CollidingEntities, Entity, &AttackableFrom), With<Health>>,
    time: Res<Time>,
    mut event_writer: EventWriter<EntityDamageEvent>,
    mut attack_entitys: Query<(&mut DamageTimer, &Damage, &LayerType)>,
) {
    for (colliding_entities, damageable_entity, attackable_from) in damageable_entities.iter_mut() {
        for colliding_entity in &colliding_entities.0 {
            if let Ok((mut timer, damage, layer_type)) = attack_entitys.get_mut(*colliding_entity) {
                if !attackable_from.0.contains(&layer_type.0) {
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
        } else {
            *health -= entity_damage_event.damage;

            // println!("{:?}: {}", entity_damage_event.entity, health)
        }
    }
}

#[derive(Event)]
pub struct EntityDeathEvent(Entity);

pub fn remove_dead_entities(
    mut event_reader: EventReader<EntityDeathEvent>,
    mut commands: Commands,
) {
    for dead_entity in event_reader.read() {
        commands.entity(dead_entity.0).despawn();
    }
}
