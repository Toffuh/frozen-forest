use crate::player::Player;
use bevy::app::App;
use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;
use bevy_xpbd_2d::prelude::{Position, PreviousPosition, RigidBody};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, follow_player);
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
    time: Res<Time>,
    player_query: Query<(&Position, &PreviousPosition), With<Player>>,
    mut camera_query: Query<(&Transform, &mut LinearVelocity), (With<Camera>, Without<Player>)>,
) {
    let Ok((player_position, player_previous_position)) = player_query.get_single() else {
        return;
    };

    let Ok((camera_transform, mut camera_velocity)) = camera_query.get_single_mut() else {
        return;
    };

    let predicted_position = player_position.0
        + ((player_position.0 - player_previous_position.0) / time.delta_seconds()) * 20.;

    let change = predicted_position - camera_transform.translation.xy();

    camera_velocity.x = change.x;
    camera_velocity.y = change.y;
}
