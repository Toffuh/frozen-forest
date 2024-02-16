use crate::entities::entity::EntityPlugin;
use crate::entities::event::EventPlugin;
use crate::entities::mob::MobPlugin;
use crate::entities::player::PlayerPlugin;
use crate::entities::spells::SpellPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::PluginGroup;

pub mod entity;
pub mod mob;
pub mod player;

pub mod data;

pub mod event;
mod spells;

pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EventPlugin)
            .add(EntityPlugin)
            .add(MobPlugin)
            .add(PlayerPlugin)
            .add(SpellPlugin)
    }
}
