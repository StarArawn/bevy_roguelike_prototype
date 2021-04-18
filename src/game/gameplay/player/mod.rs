use bevy::{prelude::*, render::texture};

mod movement;
mod player;
pub use player::Player;

pub use movement::movement;

use crate::game::GameState;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn()
        .insert(Player::default());

    let map_player_texture_handle = asset_server.load("textures/player_sprite.png");
    let map_player_sprite_material = materials.add(map_player_texture_handle.into());
    commands.spawn_bundle(SpriteBundle {
        material: map_player_sprite_material,
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..Default::default()
    })
    .insert(Player::default());

}


pub fn spawn_player_battleview(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let battle_player_texture_handle = asset_server.load("textures/characters/huntress/idle.png");
    let texture_atlas = TextureAtlas::from_grid(battle_player_texture_handle, Vec2::new(150.0, 150.0), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::from_scale(Vec3::splat(5.0));
    transform.translation.z = 12.0;

    parent.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform,
        ..Default::default()
    })
    .insert(Timer::from_seconds(0.1, true));
}