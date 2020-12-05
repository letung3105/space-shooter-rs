use crate::{
    components::{Animation, HitBox, ShipLaser, TimeToLive, Velocity},
    constant::{
        ANIMATION_INTERVAL, SHIP_LASER_INITIAL_VELOCITY, SHIP_LASER_SPRITE_HEIGHT,
        SHIP_LASER_SPRITE_WIDTH, SHIP_LASER_TIME_TO_LIVE_DURATION,
    },
    events::SpawnShipLaserEvent,
    resource::{EventReaders, TextureAtlasHandles},
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ShipLaserComponents {
    pub ship_laser: ShipLaser,
    pub time_to_live: TimeToLive,
    pub velocity: Velocity,
    pub hit_box: HitBox,
    pub animation: Animation,
}

pub fn spawn_ship_laser(
    mut commands: Commands,
    spawn_ship_laser_events: Res<Events<SpawnShipLaserEvent>>,
    texture_atlas_handles: Res<TextureAtlasHandles>,
    mut event_readers: ResMut<EventReaders>,
) {
    for evt in event_readers
        .spawn_ship_laser
        .iter(&spawn_ship_laser_events)
    {
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas_handles.laser_bolts.clone(),
                transform: Transform {
                    translation: evt.laser_translation,
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(1),
                ..Default::default()
            })
            .with_bundle(ShipLaserComponents {
                ship_laser: ShipLaser,
                time_to_live: TimeToLive(Timer::new(SHIP_LASER_TIME_TO_LIVE_DURATION, false)),
                hit_box: HitBox(Vec2::new(SHIP_LASER_SPRITE_WIDTH, SHIP_LASER_SPRITE_HEIGHT)),
                velocity: Velocity(Vec2::new(
                    SHIP_LASER_INITIAL_VELOCITY.0,
                    SHIP_LASER_INITIAL_VELOCITY.1,
                )),
                animation: Animation {
                    idx_delta: 2,
                    sprite_count: 4,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                },
            });
    }
}