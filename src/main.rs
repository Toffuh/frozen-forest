#![allow(clippy::type_complexity)]

use crate::camera::CameraPlugin;
use crate::entities::EntityPlugins;
use crate::ui::UIPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy_xpbd_2d::plugins::PhysicsPlugins;

pub mod camera;
pub mod entities;
pub mod ui;
pub mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(EntityPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UIPlugin)
        .insert_resource(Msaa::default())
        .run();
}
