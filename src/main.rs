use crate::entity::EntityPlugin;
use crate::mob::MobPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy_xpbd_2d::plugins::PhysicsPlugins;

mod entity;
mod mob;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(MobPlugin)
        .add_plugins(EntityPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Msaa::default())
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
