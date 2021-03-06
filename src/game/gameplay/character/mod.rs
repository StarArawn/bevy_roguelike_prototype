use bevy::{prelude::*, render::camera::RenderLayers};
// use super::stats::{Stat, StatName};

mod movement;
mod character;
pub mod collision;
pub use character::Character;
pub use movement::movement;

use super::attributes::*;

#[derive(Default)]
pub struct PlayerSprite {
    // Current index of the road path position that the player is traveling to.
    pub current_position: usize,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let character = commands.spawn()
        .insert(Character::default())
        .id();
    build_basic_character_attributes(&mut commands, character);

    let map_player_texture_handle = asset_server.get_handle("textures/player_sprite.png");
    let map_player_sprite_material = materials.add(map_player_texture_handle.into());
    commands
        .spawn_bundle(SpriteBundle {
            material: map_player_sprite_material,
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        })
        .insert(PlayerSprite::default())
        .insert(RenderLayers::layer(0));


    let battle_player_texture_handle = asset_server.get_handle("textures/characters/huntress/idle.png");
    let texture_atlas =
        TextureAtlas::from_grid(battle_player_texture_handle, Vec2::new(150.0, 150.0), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut transform = Transform::from_scale(Vec3::splat(5.0));
    transform.translation.x = -800.0;
    transform.translation.z = 2.0;
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(RenderLayers::layer(1))
        .insert(Timer::from_seconds(0.1, true));
}
