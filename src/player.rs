use bevy::prelude::*;
use std::ops::Mul;

pub struct PlayerPlugin;

static PLAYER_SPEED: f32 = 500.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
pub struct Player;

pub fn player_setup(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
    ));
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
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

    let direction = direction
        .normalize_or_zero()
        .mul(time.delta().as_secs_f32())
        .mul(PLAYER_SPEED);

    if let Ok(mut transform) = player.get_single_mut() {
        transform.translation.x += direction.x;
        transform.translation.y += direction.y;
    }
}
