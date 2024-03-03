use crate::entities::entity::EntityPlugin;
use crate::entities::event::EventPlugin;
use crate::entities::mob::MobPlugin;
use crate::entities::player::attacks::AttackPlugin;
use crate::entities::player::melee::MeleePlugin;
use crate::entities::player::fireball::SpellPlugin;
use crate::entities::player::PlayerPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::PluginGroup;
use crate::entities::longtimeAttack::LongTimeAttackPlugin;

pub mod data;
pub mod entity;
pub mod event;
pub mod mob;
pub mod player;
mod longtimeAttack;

pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EventPlugin)
            .add(EntityPlugin)
            .add(MobPlugin)
            .add(PlayerPlugin)
            .add(AttackPlugin)
            .add(MeleePlugin)
            .add(SpellPlugin)
            .add(LongTimeAttackPlugin)
    }
}
