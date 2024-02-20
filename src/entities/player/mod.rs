use crate::entities::data::{
    AttackableFrom, Damage, EntityType, Health, Player, PlayerAttackCoolDown, MAX_PLAYER_HEALTH,
    PLAYER_RADIUS, PLAYER_SPEED,
};
use crate::entities::event::PlayerMoveEvent;
use crate::PhysicsLayers;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::ops::Mul;

pub mod attacks;
pub mod melee;
pub mod spells;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (handle_keyboard_input, move_player).chain());
    }
}

pub fn player_setup(mut commands: Commands) {
    commands.spawn((
        Player,
        EntityType::Player,
        //add here all layers which can make damage to a player
        AttackableFrom(vec![EntityType::Mob]),
        Damage(1.),
        Health(MAX_PLAYER_HEALTH),
        PlayerAttackCoolDown::default(),
        RigidBody::Dynamic,
        Restitution::new(0.),
        Collider::circle(PLAYER_RADIUS),
        CollisionLayers::new(
            [PhysicsLayers::Player, PhysicsLayers::Entity],
            LayerMask::ALL,
        ),
        LinearVelocity(vec2(0., 0.)),
        LinearDamping(20.),
        LockedAxes::ROTATION_LOCKED,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(vec2(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.)),
                ..default()
            },
            transform: Transform::from_translation(vec3(0., 0., 0.)),
            ..default()
        },
    ));
}

pub fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
) {
    let mut direction = Vec2::new(0., 0.);

    if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        direction.x -= 1.;
    }

    if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        direction.x += 1.;
    }

    if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        direction.y += 1.;
    }

    if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        direction.y -= 1.;
    }

    if direction.length() == 0. {
        return;
    }

    player_move_event.send(PlayerMoveEvent(direction));
}

pub fn move_player(
    mut player_move_events: EventReader<PlayerMoveEvent>,
    mut player: Query<&mut LinearVelocity, With<Player>>,
) {
    let Ok(mut velocity) = player.get_single_mut() else {
        return;
    };

    for player_move_event in player_move_events.read() {
        let direction = player_move_event.0.normalize_or_zero().mul(PLAYER_SPEED);

        velocity.x = direction.x;
        velocity.y = direction.y;
    }
}
