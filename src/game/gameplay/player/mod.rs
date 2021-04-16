use bevy::prelude::*;

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

    let texture_handle: Handle<Texture> = asset_server.load("textures/player_sprite.png");

    let player_sprite_material = materials.add(texture_handle.into());

    commands
        .spawn_bundle(SpriteBundle {
            material: player_sprite_material,
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        })
        .insert(GameState::MapView)
        .insert(Player::default());
}