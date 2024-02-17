use crate::entities::data::{
    AttackTimer, Damage, EntityType, Fireball, Player, FIRE_BALL_DAMAGE, FIRE_BALL_RADIUS,
    FIRE_BALL_SPEED,
};
use crate::entities::event::EntityDeathEvent;
use crate::entities::player::attacks::PlayerAttackEvent;
use crate::ui::{InventorySlot, SelectedSlot, AttackType};
use crate::PhysicsLayers;
use bevy::app::{App, Update};
use bevy::input::Input;
use bevy::math::{vec2, vec3};
use bevy::prelude::{
    default, Camera, Color, Commands, Entity, EventReader, EventWriter, GlobalTransform,
    MouseButton, Plugin, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Window, With,
};
use bevy_xpbd_2d::components::{
    Collider, CollisionLayers, LinearDamping, LinearVelocity, LockedAxes, Restitution, RigidBody,
};
use bevy_xpbd_2d::prelude::CollidingEntities;
use iter_tools::Itertools;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_fire_ball, remove_fireball_on_collision));
    }
}

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
            let offset = direction * 50.;

            let fire_ball_translation = player_position + offset;

            commands.spawn((
                Fireball(),
                EntityType::Spell,
                Damage(FIRE_BALL_DAMAGE as f64),
                RigidBody::Dynamic,
                AttackTimer::new_attack_timer(0.),
                Restitution::new(0.),
                Collider::ball(FIRE_BALL_RADIUS),
                CollisionLayers::all_masks::<PhysicsLayers>().add_group(PhysicsLayers::Spell),
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
                        fire_ball_translation.x,
                        fire_ball_translation.y,
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
    colliding_entities: Query<(&CollidingEntities, Entity), With<Fireball>>,
) {
    for (collding, entity) in colliding_entities.iter() {
        if !collding.0.is_empty() {
            event_writer.send(EntityDeathEvent(entity));
        }
    }
}
