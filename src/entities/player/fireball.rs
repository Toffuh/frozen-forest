use crate::entities::data::{AttackTimer, Damage, EntityType, Fireball, Player, FIRE_BALL_DAMAGE, FIRE_BALL_RADIUS, FIRE_BALL_SPEED, DespawnTimer};
use crate::entities::event::{EntityDeathEvent};
use crate::entities::player::attacks::{ PlayerAttackEvent};
use crate::ui::AttackType;
use crate::PhysicsLayers;
use bevy::app::{App, Update};

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, CollisionLayers, LinearDamping, LinearVelocity, LockedAxes, Restitution, RigidBody, Sensor};
use bevy_xpbd_2d::prelude::CollidingEntities;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use iter_tools::Itertools;
use crate::entities::longtimeAttack::LongTimeAttack;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_fire_ball, remove_fireball_on_collision, spawn_fireball_explosion))
            .add_event::<FireballExplosionEvent>();
    }
}

#[derive(Event, PartialEq)]
pub struct FireballExplosionEvent(Vec2);

pub fn spawn_fire_ball(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    player_query: Query<&Transform, With<Player>>,
    mut attack_event: EventReader<PlayerAttackEvent>,
) {
    if !attack_event
        .read()
        .contains(&PlayerAttackEvent(AttackType::Fireball))
    {
        return;
    }

    attack_event.clear();

    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        if let Ok(player_transform) = player_query.get_single() {
            let player_position = Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            );
            let cursor_position = Vec2::new(world_position.x, world_position.y);
            let direction = (cursor_position - player_position).normalize_or_zero();

            commands.spawn((
                Fireball(),
                EntityType::Spell,
                RigidBody::Dynamic,
                Restitution::new(0.),
                Collider::circle(FIRE_BALL_RADIUS),
                CollisionLayers::new(
                    PhysicsLayers::Spell,
                    [
                        PhysicsLayers::Mob,
                        PhysicsLayers::Wall,
                        PhysicsLayers::ClosedTile,
                    ],
                ),
                LinearVelocity(Vec2::new(direction.x, direction.y) * FIRE_BALL_SPEED),
                LinearDamping(0.),
                LockedAxes::ROTATION_LOCKED,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1., 0.5, 0.),
                        custom_size: Some(vec2(FIRE_BALL_RADIUS * 2., FIRE_BALL_RADIUS * 2.)),
                        ..default()
                    },
                    transform: Transform::from_translation(vec3(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        0.,
                    )),
                    ..default()
                },
            ));
        }
    }
}

pub fn remove_fireball_on_collision(
    mut event_writer: EventWriter<EntityDeathEvent>,
    mut fireball_explosion_event: EventWriter<FireballExplosionEvent>,
    fireballs: Query<(&CollidingEntities, Entity, &Transform), With<Fireball>>,
) {
    for (collding_entitys, fireball_entity, fireball_transform) in fireballs.iter() {
        if !collding_entitys.0.is_empty() {
            event_writer.send(EntityDeathEvent(fireball_entity));

            fireball_explosion_event.send(FireballExplosionEvent(fireball_transform.translation.xy()));
        }
    }
}

fn spawn_fireball_explosion(
    mut commands: Commands,
    mut attack_event: EventReader<FireballExplosionEvent>,
) {
    for event in attack_event.read() {
        let fireball_pos = event.0;

        commands.spawn((
            LongTimeAttack {
                damaged_entities: vec![],
            },
            Damage(FIRE_BALL_DAMAGE as f64),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 1., 0.),
                    custom_size: Some(vec2(50., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(fireball_pos.x, fireball_pos.y, 0.),
                ..default()
            },
            DespawnTimer::from_seconds(0.2),
            Sensor,
            CollisionLayers::new([PhysicsLayers::Fireball], [PhysicsLayers::Mob]),
            Collider::ball(25.),
            RigidBody::Static,
        ));
    }
}
