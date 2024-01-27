use crate::mob::MobPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

use crate::camera::CameraPlugin;
use bevy::prelude::*;
use bevy_xpbd_2d::plugins::PhysicsPlugins;

mod camera;
mod damage;
mod health;
mod mob;
mod player;
mod world;

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
