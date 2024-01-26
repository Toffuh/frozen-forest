use crate::mob::MobPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_xpbd_2d::plugins::PhysicsPlugins;
use crate::camera::CameraPlugin;

mod damage;
mod health;
mod mob;
mod player;
mod world;
mod camera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(MobPlugin)
        .add_plugins(CameraPlugin)
        .run();
}