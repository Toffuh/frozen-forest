use crate::entities::data::{Damage, DespawnTimer, Player, PLAYER_DAMAGE, PLAYER_RADIUS};

use crate::entities::player::attacks::PlayerAttackEvent;

use crate::ui::AttackType;
use crate::PhysicsLayers;
use bevy::app::{App, Plugin, Update};

use crate::entities::longtime_attack::LongTimeAttack;
use bevy::math::{vec2, Vec2};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use iter_tools::Itertools;

pub struct MeleePlugin;

impl Plugin for MeleePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_player_attack);
    }
}

fn spawn_player_attack(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut attack_event: EventReader<PlayerAttackEvent>,
) {
    if !attack_event
        .read()
        .contains(&PlayerAttackEvent(AttackType::Melee))
    {
        return;
    }

    attack_event.clear();

    let player_transform = player_query.single();
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let world_pos = world_position.xy();
        let player_pos = player_transform.translation.xy();

        let dir = (world_pos - player_pos).normalize_or_zero();

        let collider_pos = (dir * (PLAYER_RADIUS * 2.)).xy() + player_pos;

        commands.spawn((
            LongTimeAttack {
                damaged_entities: vec![],
            },
            Damage(PLAYER_DAMAGE),
            Rotation::from_radians(Vec2::X.angle_between(dir)),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.7, 0.25, 0.2),
                    custom_size: Some(vec2(12., 30.)),
                    ..default()
                },
                transform: Transform::from_xyz(collider_pos.x, collider_pos.y, 0.),
                ..default()
            },
            DespawnTimer::from_seconds(0.2),
            Sensor,
            CollisionLayers::new([PhysicsLayers::Player], [PhysicsLayers::Mob]),
            Collider::capsule(18., 6.),
            RigidBody::Static,
        ));
    }
}
