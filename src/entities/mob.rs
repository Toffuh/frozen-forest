use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::{vec2, Vec2};
use bevy::prelude::{
    default, Color, Commands, Query, Sprite, SpriteBundle, Transform, With, Without,
};

use crate::entities::data::{
    AttackTimer, AttackableFrom, Damage, EntityType, Health, Mob, Player, MOB_RADIUS, MOB_SPEED,
};
use bevy::window::{PrimaryWindow, Window};
use bevy_xpbd_2d::components::{Collider, LinearDamping, LinearVelocity, LockedAxes, RigidBody};
use bevy_xpbd_2d::prelude::{ColliderDensity, Restitution};
use rand::Rng;
use std::ops::Mul;

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_mob)
            .add_systems(Startup, spawn_mobs);
    }
}

pub fn spawn_mobs(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let min_x = -(window.width() / 2.);
    let max_x = window.width() / 2.;

    let mut random = rand::thread_rng();

    for _i in 0..2 {
        let random_x = random.gen_range(min_x..max_x);

        commands.spawn((
            Mob,
            EntityType::Mob,
            AttackableFrom(vec![EntityType::Player]),
            Damage(1.0),
            Health(10.0),
            AttackTimer::default(),
            RigidBody::Dynamic,
            Restitution::new(0.),
            Collider::ball(MOB_RADIUS),
            LinearVelocity(vec2(0., 0.)),
            LinearDamping(20.),
            LockedAxes::ROTATION_LOCKED,
            ColliderDensity(0.),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.75, 0.25),
                    custom_size: Some(Vec2::new(MOB_RADIUS * 2., MOB_RADIUS * 2.)),
                    ..default()
                },
                transform: Transform::from_xyz(random_x, -window.height() / 2. + 50., 0.),
                ..default()
            },
        ));
    }
}

pub fn move_mob(
    mut mob_query: Query<(&mut LinearVelocity, &Transform), (With<Mob>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut linear_velocity, transform) in mob_query.iter_mut() {
            let vec = (player_transform.translation - transform.translation)
                .normalize_or_zero()
                .mul(MOB_SPEED);

            linear_velocity.x = vec.x;
            linear_velocity.y = vec.y;
        }
    }
}
