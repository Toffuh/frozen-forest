use bevy::math::Vec2;
use bevy::prelude::{Component, Entity, Event, Timer, TimerMode};

pub static ATTACK_COOLDOWN: f32 = 1.;

pub static MOB_SPEED: f32 = 200.;

#[derive(Component)]
pub struct Damage(pub f64);

#[derive(Component)]
pub struct Health(pub f64);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mob;
#[derive(PartialEq, Component)]
pub enum EntityType {
    Player,
    Mob,
    Wall,
}
#[derive(Component)]
pub struct AttackableFrom(pub Vec<EntityType>);

#[derive(Component)]
pub struct AttackTimer(pub Timer);
impl Default for AttackTimer {
    fn default() -> AttackTimer {
        AttackTimer(Timer::from_seconds(ATTACK_COOLDOWN, TimerMode::Repeating))
    }
}