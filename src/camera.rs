use crate::player::{Player, PlayerMovementSet};
use bevy::app::App;
use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;
use bevy_xpbd_2d::prelude::RigidBody;
use bevy_xpbd_2d::PhysicsSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            follow_player
                .after(PlayerMovementSet::PlayerMovement)
                .before(PhysicsSet::Prepare),
        );
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        RigidBody::Kinematic,
        LinearVelocity::ZERO,
    ));
}

pub fn follow_player(
    player_query: Query<(&Transform, &LinearVelocity), With<Player>>,
    mut camera_query: Query<(&Transform, &mut LinearVelocity), (With<Camera>, Without<Player>)>,
) {
    let Ok((player_transform, player_velocity)) = player_query.get_single() else {
        return;
    };

    let Ok((camera_transform, mut camera_velocity)) = camera_query.get_single_mut() else {
        return;
    };

    let predicted_position = player_transform.translation.xy() + player_velocity.0 * 1.2;

    let change = predicted_position - camera_transform.translation.xy();

    camera_velocity.x = change.x;
    camera_velocity.y = change.y;
}
