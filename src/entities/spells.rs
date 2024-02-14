use std::ops::Mul;
use bevy::app::{App, Update};
use bevy::input::Input;
use bevy::math::{vec2, vec3, Vec3};
use bevy::prelude::{Camera, Color, Commands, default, Entity, EventWriter, GlobalTransform, MouseButton, Plugin, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Window, With};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, LinearDamping, LinearVelocity, LockedAxes, Restitution, RigidBody};
use bevy_xpbd_2d::prelude::CollidingEntities;
use crate::entities::data::{Damage, EntityType, FIRE_BALL_RADIUS, FIRE_BALL_SPEED, Fireball, Player, AttackTimer, AttackTimerInit, FIRE_BALL_DAMAGE};
use crate::entities::event::EntityDeathEvent;
use crate::PhysicsLayers;
use crate::ui::{InventorySlot, SelectedSlot, SpellType};

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire_ball_setup, remove_fireball_on_collision));
    }
}

pub fn fire_ball_setup(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    selected_inventory_slot: Res<SelectedSlot>,
    inventory: Query<&InventorySlot>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    };

    for inventory_slot in inventory.iter() {
        if inventory_slot.spell == SpellType::Fireball {
            if inventory_slot.index != selected_inventory_slot.index {
                return;
            }
        }
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let world_pos = Vec3::new(world_position.x, world_position.y, 0.);

        if let Ok(player_transform) = player_query.get_single() {
            let vec = (world_pos - player_transform.translation)
                .normalize_or_zero()
                .mul(FIRE_BALL_SPEED);

            commands.spawn((
                Fireball(),
                EntityType::Spell,
                Damage(FIRE_BALL_DAMAGE.into()),
                RigidBody::Dynamic,
                AttackTimer::new_attack_timer(0.),
                Restitution::new(0.),
                Collider::ball(FIRE_BALL_RADIUS),
                CollisionLayers::all_masks::<PhysicsLayers>().add_group(PhysicsLayers::Spell),
                LinearVelocity(Vec2::new(vec.x, vec.y)),
                LinearDamping(0.),
                LockedAxes::ROTATION_LOCKED,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1., 0.5, 0.),
                        custom_size: Some(vec2(FIRE_BALL_RADIUS * 2., FIRE_BALL_RADIUS * 2.)),
                        ..default()
                    },
                    transform: Transform::from_translation(vec3(world_pos.x, world_pos.y, 0.)),
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
        if collding.0.len() != 0 {
            event_writer.send(EntityDeathEvent(entity));
        }
    }
}