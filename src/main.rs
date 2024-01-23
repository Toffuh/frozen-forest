use crate::mob::MobPlugin;
use crate::player::PlayerPlugin;
use bevy::prelude::*;

mod mob;
pub mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(MobPlugin)
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
