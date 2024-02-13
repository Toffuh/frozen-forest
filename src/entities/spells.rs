use std::ops::Mul;
use std::slice::Windows;
use bevy::app::{App, Startup, Update};
use bevy::input::Input;
use bevy::math::{vec2, Vec2Swizzles, vec3, Vec3};
use bevy::prelude::{Camera, Color, Commands, default, GlobalTransform, MouseButton, Plugin, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Window, With};
use bevy_xpbd_2d::components::{Collider, CollisionLayers, LinearDamping, LinearVelocity, LockedAxes, Restitution, RigidBody};
use crate::entities::data::{Damage, EntityType, FIRE_BALL_RADIUS, FIRE_BALL_SPEED, Mob, MOB_SPEED, PLAYER_RADIUS, Fireball, Player};
use crate::PhysicsLayers;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire_ball_setup);
        // .add_systems(Update, fire_ball_move_system);
    }
}

pub fn fire_ball_setup(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    };

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
                Fireball(false),
                EntityType::Spell,
                Damage(5.),
                RigidBody::Dynamic,
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

// fn mouse_click_spawn_fireball(
//     mut commands: Commands,
//     mouse_button_input: Res<Input<MouseButton>>,
//     windows: Res<Windows>,
// ) {
//     if mouse_button_input.just_pressed(MouseButton::Left) {
//         if let Some(window) = windows.get_primary() {
//             if let Some(cursor_position) = window.cursor_position() {
//                 let spawn_position = Vec3::new(cursor_position.x as f32, cursor_position.y as f32, 0.0);
//                 commands.spawn().insert_bundle(SpriteBundle {
//                     material: ColorMaterial {
//                         color: Color::rgb(1., 0.5, 0.),
//                         texture: None,
//                     },
//                     transform: Transform::from_translation(spawn_position),
//                     ..Default::default()
//                 })
//                     .insert(Fireball)
//                     .insert(RigidBody::Dynamic)
//                     .insert(LinearVelocity(Vec2::ZERO))
//                     .insert(Collider::Ball { radius: 10.0 });
//             }
//         }
//     }
// }

// pub fn fire_ball_move_system(
//     mouse_button_input: Res<Input<MouseButton>>,
//     windows: Query<&Window>,
//     camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
//     mut fire_ball_query: Query<(&mut LinearVelocity, &Transform, &mut Fireball)>,
// ) {
//     if !mouse_button_input.just_pressed(MouseButton::Left) {
//         return;
//     };
//
//     for (mut linear_velocity, transform, mut fireball) in fire_ball_query.iter_mut() {
//         if fireball.0 {
//             return;
//         }
//
//         fireball.0 = true;
//
//         let window = windows.single();
//         let (camera, camera_transform) = camera_query.single();
//
//         if let Some(world_position) = window
//             .cursor_position()
//             .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
//         {
//             let world_pos = Vec3::new(world_position.x, world_position.y, 0.);
//
//             let vec = (world_pos - transform.translation)
//                 .normalize_or_zero()
//                 .mul(FIRE_BALL_SPEED);
//
//             linear_velocity.x = vec.x;
//             linear_velocity.y = vec.y;
//         }
//     }
// }