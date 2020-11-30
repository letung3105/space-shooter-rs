use bevy::prelude::*;

/// Different states of the  when moving left/right
#[derive(Debug, PartialEq)]
pub enum ShipAnimationState {
    FullLeft,
    HalfLeft,
    Stabilized,
    HalfRight,
    FullRight,
}

/// Component that marks an entity to be a ship in the game
#[derive(Debug)]
pub struct Ship {
    pub move_speed: f32,
    pub animation_state: ShipAnimationState,
    pub laser_cooldown_timer: Timer,
    pub transition_timer: Timer,
}

/// The type of enemy
#[derive(Debug, PartialEq, Clone)]
pub enum EnemyVariant {
    Small,
    Medium,
    Big,
}

/// Component marks an entity to be an enemy
#[derive(Debug)]
pub struct Enemy {
    pub variant: EnemyVariant,
}

/// Component determines the spawn rate and spawn probability of entities
#[derive(Debug)]
pub struct EnemySpawner {
    pub timer: Timer,
    pub weights: Vec<(String, u8)>,
}

#[derive(Debug)]
pub struct ShipLaser;

#[derive(Debug)]
pub struct Velocity(pub Vec2);

/// Component that determines when to change the sprite and which sprite index to change to when
/// doing simple animation
#[derive(Debug)]
pub struct Animation {
    pub idx_delta: u32,
    pub sprite_count: u32,
    pub timer: Timer,
}

/// Component that determines the smallest possible box that includes rendered the entity.
#[derive(Debug, Clone)]
pub struct HitBox {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub struct TimeToLive(pub Timer);
