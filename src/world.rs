use crate::entity::{AttackableFrom, EntityTypes, Health};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Health(20.),
        EntityTypes::Wall,
        AttackableFrom(vec![EntityTypes::Mob]),
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
}
