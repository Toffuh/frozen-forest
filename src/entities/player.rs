use crate::entities::data::{
    AttackableFrom, Damage, EntityType, Health, Player, PlayerAttackTimer, MAX_PLAYER_HEALTH,
    PLAYER_RADIUS, PLAYER_SPEED,
};
use crate::entities::event::{EntityDamageEvent, PlayerMoveEvent};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::ops::Mul;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (handle_keyboard_input, move_player).chain())
            .add_systems(Update, player_attack_system);
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

pub fn player_attack_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    spatial_query: SpatialQuery,
    mut player_query: Query<
        (
            Entity,
            &Transform,
            &EntityType,
            &Damage,
            &mut PlayerAttackTimer,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    attackable_from: Query<&AttackableFrom>,
    mut event_writer: EventWriter<EntityDamageEvent>,
) {
    let (player_entity, player_transform, player_entity_type, player_damage, mut player_timer) =
        player_query.get_single_mut().unwrap();

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

        let intersections = spatial_query.shape_intersections(
            &Collider::capsule(30., 25.),
            ((world_pos - player_pos).normalize_or_zero() * (PLAYER_RADIUS + 20.)).xy()
                + player_pos,
            (world_pos - player_pos).y.atan2((world_pos - player_pos).x),
            SpatialQueryFilter::new().without_entities([player_entity]),
        );

        intersections
            .iter()
            .filter_map(|entity| {
                attackable_from
                    .get(*entity)
                    .ok()
                    .map(|attackable_from| (entity, attackable_from))
            })
            .filter(|(_, attackable_from)| attackable_from.0.contains(player_entity_type))
            .for_each(|(entity, _)| {
                event_writer.send(EntityDamageEvent {
                    entity: *entity,
                    damage: player_damage.0,
                });

                player_timer.0.reset();
            });
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
