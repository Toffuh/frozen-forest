#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::camera::CameraPlugin;
use crate::entities::EntityPlugins;
use crate::ui::UIPlugin;
use crate::world::WorldPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_xpbd_2d::plugins::*;
use bevy_xpbd_2d::prelude::*;

pub mod camera;
pub mod entities;
pub mod ui;
pub mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
        ))
        .add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
        .add_plugins(EntityPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UIPlugin)
        .insert_resource(Msaa::default())
        .insert_resource(Gravity::ZERO)
        .run();
}

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
    Player,
    Mob,
    Entity,
    Wall,
    ClosedTile,
    Spell,
}
