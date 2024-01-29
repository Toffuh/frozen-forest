use crate::entity::{AttackableFrom, Damage, DamageTimer, EntityTypes, Health};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::ops::Mul;

pub struct PlayerPlugin;

static PLAYER_SPEED: f32 = 500.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (handle_keyboard_input, move_player))
            .add_event::<PlayerMoveEvent>();
    }
}

#[derive(Component)]
pub struct Player;

pub fn player_setup(mut commands: Commands) {
    commands.spawn((
        Player,
        EntityTypes::Player,
        //add here all layers which can make damage to a player
        AttackableFrom(vec![EntityTypes::Mob]),
        Damage(1.),
        Health(30.),
        DamageTimer::default(),
        RigidBody::Dynamic,
        Restitution::new(0.),
        Collider::cuboid(50., 100.),
        LinearVelocity(vec2(0., 0.)),
        LinearDamping(20.),
        LockedAxes::ROTATION_LOCKED,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(vec2(50., 100.)),
                ..default()
            },
            transform: Transform::from_translation(vec3(0., 0., 0.)),
            ..default()
        },
    ));
}

#[derive(Event)]
pub struct PlayerMoveEvent(Vec2);

pub fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
) {
    let mut direction = Vec2::new(0., 0.);

    if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
        direction.x -= 1.;
    }

    if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
        direction.x += 1.;
    }

    if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
        direction.y += 1.;
    }

    if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
        direction.y -= 1.;
    }

    if direction.length() == 0. {
        return;
    }

    player_move_event.send(PlayerMoveEvent(direction))
}

pub fn move_player(
    mut player_move_events: EventReader<PlayerMoveEvent>,
    mut player: Query<&mut LinearVelocity, With<Player>>,
) {
    if let Ok(mut velocity) = player.get_single_mut() {
        for player_move_event in player_move_events.read() {
            let direction = player_move_event.0.normalize_or_zero().mul(PLAYER_SPEED);

            velocity.x = direction.x;
            velocity.y = direction.y;
        }
    }
}
