use crate::constant::{
    ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT,
    ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT, ENEMY_SMALL_SPRITE_WIDTH,
    EXPLOSION_SPRITE_HEIGHT, EXPLOSION_SPRITE_WIDTH, LASER_BOLT_SPRITE_HEIGHT,
    LASER_BOLT_SPRITE_WIDTH,
};
use bevy::prelude::*;
use std::collections::HashMap;

/// A structure for holding general game states that are shared across multiple systems
#[derive(Default)]
pub struct GameState {
    pub texture_atlas_handles: HashMap<String, Handle<TextureAtlas>>,
}

pub fn initialize_texture_atlases(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // LASER BOLTS SPRITESHEET
    let mut texture_atlas = TextureAtlas::from_grid(
        asset_server.load("spritesheets/laser-bolts.png"),
        Vec2::new(LASER_BOLT_SPRITE_WIDTH, LASER_BOLT_SPRITE_HEIGHT),
        2,
        2,
    );
    // Texture is row-major order, swap to turn into column-major order
    texture_atlas.textures.swap(1, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    game_state
        .texture_atlas_handles
        .insert("laser-bolts".to_string(), texture_atlas_handle);

    // EXPLOSION SPRITESHEET
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("spritesheets/explosion.png"),
        Vec2::new(EXPLOSION_SPRITE_WIDTH, EXPLOSION_SPRITE_HEIGHT),
        2,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    game_state
        .texture_atlas_handles
        .insert("explosion".to_string(), texture_atlas_handle);

    // SMALL ENEMY SPRITESHEET
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("spritesheets/enemy-small.png"),
        Vec2::new(ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT),
        2,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    game_state
        .texture_atlas_handles
        .insert("enemy-small".to_string(), texture_atlas_handle);

    // MEDIUM ENEMY SPRITESHEET
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("spritesheets/enemy-medium.png"),
        Vec2::new(ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT),
        2,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    game_state
        .texture_atlas_handles
        .insert("enemy-medium".to_string(), texture_atlas_handle);

    // BIG ENEMY SPRITESHEET
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("spritesheets/enemy-big.png"),
        Vec2::new(ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT),
        2,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    game_state
        .texture_atlas_handles
        .insert("enemy-big".to_string(), texture_atlas_handle);
}
