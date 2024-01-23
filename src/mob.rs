use bevy::app::{App, Plugin, Update};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Color, Commands, Component, Query, Res, ResMut, Resource, Sprite, SpriteBundle, Time,
    Timer, Transform, With,
};
use bevy::time::TimerMode;
use bevy::window::{PrimaryWindow, Window};
use rand::Rng;

static MOB_SPAWN_TIME: f32 = 5.;

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_mob);
        app.add_systems(Update, spawn_mobs_over_time);
        app.init_resource::<MobSpawnTimer>();
        app.add_systems(Update, tick_mob_spawn_timer);
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
    if mob_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        for i in 0..3 {
            let min_x = -(window.width() / 2.);
            let max_x = window.width() / 2.;

            let random_x = rand::thread_rng().gen_range(min_x..max_x);

            commands.spawn((
                Mob,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.75, 0.25),
                        custom_size: Some(Vec2::new(25.0, 50.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        random_x,
                        -window.height() / 2. + 50.,
                        0.,
                    )),
                    ..default()
                },
            ));
        }
    }
}

#[derive(Component)]
pub struct Mob;

pub fn move_mob(time: Res<Time>, mut query: Query<(&Mob, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        let translation = Vec3::new(0., 0., 0.);

        let translation = transform
            .translation
            .lerp(translation, time.delta_seconds());

        transform.translation = translation;
    }
}
