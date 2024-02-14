use crate::entities::data::{
    AttackableFrom, Damage, DespawnTimer, EntityType, Health, Player, PlayerAttackTimer,
    MAX_PLAYER_HEALTH, PLAYER_RADIUS, PLAYER_SPEED,
};
use crate::entities::event::{EntityDamageEvent, PlayerMoveEvent};
use crate::PhysicsLayers;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::ops::Mul;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (handle_keyboard_input, move_player).chain())
            .add_systems(Update, (player_attack_system, damage_entities));
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
        PlayerAttackTimer::default(),
        RigidBody::Dynamic,
        Restitution::new(0.),
        Collider::ball(PLAYER_RADIUS),
        CollisionLayers::all_masks::<PhysicsLayers>()
            .add_groups([PhysicsLayers::Player, PhysicsLayers::Entity]),
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

#[derive(Component)]
pub struct PlayerAttack {
    damaged_entities: Vec<Entity>,
}

pub fn player_attack_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut PlayerAttackTimer), With<Player>>,
    time: Res<Time>,
) {
    let (player_transform, mut player_timer) = player_query.single_mut();

    player_timer.0.tick(time.delta());

    if !player_timer.0.finished() {
        return;
    }

    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    };

    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let world_pos = world_position.xy();
        let player_pos = player_transform.translation.xy();

        let dir = (world_pos - player_pos).normalize_or_zero();

        let collider_pos = (dir * (PLAYER_RADIUS + 30.)).xy() + player_pos;

        commands.spawn((
            PlayerAttack {
                damaged_entities: vec![],
            },
            Rotation::from_radians(Vec2::X.angle_between(dir)),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.7, 0.25, 0.2),
                    custom_size: Some(vec2(30., 70.)),
                    ..default()
                },
                transform: Transform::from_xyz(collider_pos.x, collider_pos.y, 0.),
                ..default()
            },
            DespawnTimer::from_seconds(0.1),
            Sensor,
            CollisionLayers::new([PhysicsLayers::Player], [PhysicsLayers::Mob]),
            Collider::capsule(40., 15.),
            RigidBody::Static,
        ));

        player_timer.0.reset();
    }
}

pub fn damage_entities(
    mut player_attacks: Query<(&CollidingEntities, &mut PlayerAttack)>,
    player_damage: Query<&Damage, With<Player>>,
    attackable_from: Query<&AttackableFrom>,
    mut event_writer: EventWriter<EntityDamageEvent>,
) {
    let damage = player_damage.single().0;

    for (touching_entities, mut player_attack) in player_attacks.iter_mut() {
        for touching_entity in &touching_entities.0 {
            let can_be_attacked = attackable_from
                .get(*touching_entity)
                .map(|attackable_from| attackable_from.0.contains(&EntityType::Player))
                .unwrap_or(false);

            if !can_be_attacked || player_attack.damaged_entities.contains(touching_entity) {
                continue;
            }

            player_attack.damaged_entities.push(*touching_entity);
            event_writer.send(EntityDamageEvent {
                entity: *touching_entity,
                damage,
            })
        }
    }
}

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
    let Ok(mut velocity) = player.get_single_mut() else {
        return;
    };

    for player_move_event in player_move_events.read() {
        let direction = player_move_event.0.normalize_or_zero().mul(PLAYER_SPEED);

        velocity.x = direction.x;
        velocity.y = direction.y;
    }
}
