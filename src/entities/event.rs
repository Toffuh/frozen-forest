use bevy::app::{App, Plugin};
use bevy::math::Vec2;
use bevy::prelude::{Entity, Event};

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityDamageEvent>()
            .add_event::<EntityDeathEvent>()
            .add_event::<PlayerMoveEvent>();
    }
}

#[derive(Event)]
pub struct EntityDamageEvent {
    pub entity: Entity,
    pub damage: f64,
}

#[derive(Event)]
pub struct EntityDeathEvent(pub Entity);

#[derive(Event)]
pub struct PlayerMoveEvent(pub Vec2);
