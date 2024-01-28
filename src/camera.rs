use crate::player::Player;
use bevy::app::App;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, follow_player);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn follow_player(
    time: Res<Time>,
    player_query: Query<(&Transform, &LinearVelocity), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok((player_transform, player_velocity)) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // println!("{:?}", player_velocity);

            let predicted_position = player_transform.translation.xy() + player_velocity.0 * 1.3;
            let predicted_position = vec3(predicted_position.x, predicted_position.y, 0.);

            let change = (predicted_position - camera_transform.translation) * time.delta_seconds();

            camera_transform.translation += change;
        }
    }
}
