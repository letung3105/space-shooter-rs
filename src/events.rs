use crate::components::{EnemyVariant, HitBox, Velocity};
use bevy::prelude::*;
use std::time::Duration;

pub struct SpawnLaserEvent {
    pub laser_source: Entity,
    pub laser_translation: Vec3,
    pub laser_velocity: Velocity,
    pub laser_hit_box: HitBox,
    pub laser_time_to_live_duration: Duration,
    pub laser_initial_sprite_idx: u32,
}

pub struct SpawnEnemyEvent {
    pub enemy_variant: EnemyVariant,
    pub enemy_translation: Vec3,
}

pub struct SpawnExplosionEvent {
    pub explosion_translation: Vec3,
    pub explosion_time_to_live_duration: Duration,
}

pub struct CollisionLaserEnemyEvent {
    pub laser_entity: Entity,
    pub enemy_entity: Entity,
}

pub struct CollisionLaserShipEvent {
    pub laser_entity: Entity,
    pub ship_entity: Entity,
}
