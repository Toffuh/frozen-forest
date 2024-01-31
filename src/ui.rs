use crate::entities::data::{Health, Player, MAX_PLAYER_HEALTH};
use bevy::app::{App, Startup};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_health_bar)
            .add_systems(Update, update_health_bar);
    }
}

#[derive(Component)]
pub struct HealthBar;

pub fn setup_health_bar(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Px(200.),
            height: Val::Px(20.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        background_color: Color::rgb(0., 0., 0.).into(),
        ..default()
    };

    let bar = (
        HealthBar,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            background_color: Color::rgb(0.9, 0.25, 0.35).into(),
            ..default()
        },
    );

    commands.spawn(container).with_children(|parent| {
        parent.spawn(bar);
    });
}

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
) {
    let Ok(health) = player_query.get_single() else {
        warn!("could not find a single Player");
        return;
    };

    let Ok(mut heal_bar) = health_bar_query.get_single_mut() else {
        warn!("could not find a single HealthBar");
        return;
    };

    heal_bar.width = Val::Percent((health.0 / MAX_PLAYER_HEALTH) as f32 * 100.);
}
