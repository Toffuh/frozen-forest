use bevy::prelude::{Component, Timer, TimerMode};

pub static ATTACK_COOLDOWN: f32 = 1.;
pub static PLAYER_ATTACK_COOLDOWN: f32 = 0.5;

pub static MOB_SPEED: f32 = 100.;
pub static PLAYER_SPEED: f32 = 175.;
pub static MAX_PLAYER_HEALTH: f64 = 30.;
pub static PLAYER_RADIUS: f32 = 8.;
pub static MOB_RADIUS: f32 = 4.;

#[derive(Component)]
pub struct Damage(pub f64);

#[derive(Component)]
pub struct Health(pub f64);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mob;

#[derive(PartialEq, Component, Debug)]
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

#[derive(Component)]
pub struct PlayerAttackTimer(pub Timer);

impl Default for PlayerAttackTimer {
    fn default() -> PlayerAttackTimer {
        PlayerAttackTimer(Timer::from_seconds(PLAYER_ATTACK_COOLDOWN, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct DamageCoolDown(pub Timer);

impl DamageCoolDown {
    pub fn new(seconds: f32) -> DamageCoolDown {
        DamageCoolDown(Timer::from_seconds(seconds, TimerMode::Once))
    }
}
