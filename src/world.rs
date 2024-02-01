use crate::entities::data::{AttackableFrom, EntityType, Health};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub static TILE_SIZE: usize = 300;

#[derive(Component)]
pub struct Tile;

fn setup(mut commands: Commands) {
    commands.spawn((
        Health(20.),
        EntityType::Wall,
        AttackableFrom(vec![EntityType::Mob]),
        RigidBody::Static,
        Collider::cuboid(100., 100.),
        Restitution::new(0.),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.4, 0.1, 0.5),
                custom_size: Some(vec2(100., 100.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(100., 100., 0.)),
            ..default()
        },
    ));

    for x in -5..=5isize {
        for y in -5..=5isize {
            if vec2(x as f32, y as f32).length() > 4. {
                blocked_normal_tile(&mut commands, x, y);
            } else {
                normal_tile(&mut commands, x, y);
            }
        }
    }
}

fn normal_tile(commands: &mut Commands, x: isize, y: isize) {
    commands.spawn((
        Tile,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.2, 0.1),
                custom_size: Some(vec2(TILE_SIZE as f32, TILE_SIZE as f32)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                x as f32 * TILE_SIZE as f32,
                y as f32 * TILE_SIZE as f32,
                -5.,
            )),
            ..default()
        },
    ));
}

fn blocked_normal_tile(commands: &mut Commands, x: isize, y: isize) {
    commands.spawn((
        Tile,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.2, 0.1),
                custom_size: Some(vec2(TILE_SIZE as f32, TILE_SIZE as f32)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                x as f32 * TILE_SIZE as f32,
                y as f32 * TILE_SIZE as f32,
                -5.,
            )),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(TILE_SIZE as f32, TILE_SIZE as f32),
        Restitution::new(0.),
    ));
}
