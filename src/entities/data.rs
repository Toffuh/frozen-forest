use bevy::prelude::{Component, Timer, TimerMode};

pub static PLAYER_ATTACK_COOLDOWN: f32 = 0.5;

pub static MOB_SPEED: f32 = 100.;
pub static PLAYER_SPEED: f32 = 175.;
pub static MAX_PLAYER_HEALTH: f64 = 30.;
pub static PLAYER_RADIUS: f32 = 8.;
pub static MOB_RADIUS: f32 = 4.;
pub static FIRE_BALL_RADIUS: f32 = 6.;
pub static FIRE_BALL_SPEED: f32 = 300.;
pub static FIRE_BALL_DAMAGE: f32 = 4.;

#[derive(Component)]
pub struct Damage(pub f64);

#[derive(Component)]
pub struct Health(pub f64);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct Fireball();

#[derive(PartialEq, Component, Debug)]
pub enum EntityType {
    Player,
    Mob,
    Wall,
    Spell,
}

#[derive(Component)]
pub struct AttackCooldown(f32);

#[derive(Component)]
pub struct AttackableFrom(pub Vec<EntityType>);

#[derive(Component)]
pub struct AttackTimer(pub Timer);

impl Default for AttackTimer {
    fn default() -> AttackTimer {
        AttackTimer(Timer::from_seconds(1., TimerMode::Repeating))
    }
}

impl AttackTimer {
    pub fn new_attack_timer(seconds: f32) -> Self {
        AttackTimer(Timer::from_seconds(seconds, TimerMode::Repeating))
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

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

impl DespawnTimer {
    pub fn from_seconds(seconds: f32) -> Self {
        DespawnTimer(Timer::from_seconds(seconds, TimerMode::Once))
    }
}
