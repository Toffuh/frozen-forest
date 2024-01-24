use crate::player::Player;
use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::{
    default, Color, Commands, Component, Query, Res, ResMut, Resource, Sprite, SpriteBundle, Time,
    Timer, Transform, With, Without,
};
use bevy::time::TimerMode;
use bevy::window::{PrimaryWindow, Window};
use rand::Rng;
use std::ops::Mul;

static MOB_SPAWN_TIME: f32 = 5.;

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_mob)
            .add_systems(Update, spawn_mobs_over_time)
            .add_systems(Update, tick_mob_spawn_timer)
            .init_resource::<MobSpawnTimer>();
    }
}

#[derive(Resource)]
pub struct MobSpawnTimer {
    pub timer: Timer,
}

impl Default for MobSpawnTimer {
    fn default() -> MobSpawnTimer {
        MobSpawnTimer {
            timer: Timer::from_seconds(MOB_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn tick_mob_spawn_timer(mut mob_spawn_timer: ResMut<MobSpawnTimer>, time: Res<Time>) {
    mob_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_mobs_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mob_spawn_timer: Res<MobSpawnTimer>,
) {
    if !mob_spawn_timer.timer.finished() {
        return;
    }

    let window = window_query.get_single().unwrap();

    let min_x = -(window.width() / 2.);
    let max_x = window.width() / 2.;

    let mut random = rand::thread_rng();

    for _i in 0..70 {
        let random_x = random.gen_range(min_x..max_x);

        commands.spawn((
            Mob,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.75, 0.25),
                    custom_size: Some(Vec2::new(25.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(random_x, -window.height() / 2. + 50., 0.),
                ..default()
            },
        ));
    }
}

#[derive(Component)]
pub struct Mob;

pub fn move_mob(
    time: Res<Time>,
    mut mob_query: Query<(&mut Transform), (With<Mob>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut transform in mob_query.iter_mut() {
            let vec = player_transform.translation - transform.translation;

            transform.translation += vec.normalize_or_zero().mul(100.).mul(time.delta_seconds());
        }
    }
}