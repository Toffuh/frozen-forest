use bevy::prelude::*;
use std::ops::Mul;

pub struct PlayerPlugin;

static PLAYER_SPEED: f32 = 500.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (handle_keyboard_input, move_player))
            .add_event::<PlayerMoveEvent>();
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

#[derive(Event)]
pub struct PlayerMoveEvent(Vec2);

pub fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
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

    if direction.length() == 0. {
        return;
    }

    player_move_event.send(PlayerMoveEvent(direction.normalize()))
}

pub fn move_player(
    time: Res<Time>,
    mut player_move_events: EventReader<PlayerMoveEvent>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player.get_single_mut() {
        for player_move_event in player_move_events.read() {
            let direction = player_move_event
                .0
                .mul(time.delta().as_secs_f32())
                .mul(PLAYER_SPEED);

            transform.translation.x += direction.x;
            transform.translation.y += direction.y;
        }
    }
}
