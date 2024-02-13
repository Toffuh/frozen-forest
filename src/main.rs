#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::camera::CameraPlugin;
use crate::entities::EntityPlugins;
use crate::ui::UIPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy_xpbd_2d::plugins::PhysicsPlugins;
use bevy_xpbd_2d::prelude::PhysicsLayer;

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
        .add_plugins(EntityPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UIPlugin)
        // .add_plugins(PhysicsDebugPlugin::default())
        // .insert_resource(PhysicsDebugConfig::all())
        .insert_resource(Msaa::default())
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
