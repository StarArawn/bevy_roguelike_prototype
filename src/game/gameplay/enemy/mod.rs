use bevy::prelude::*;

use crate::game::GameState;

pub mod spawner;

#[derive(Default)]
pub struct Enemy {
}

pub fn spawn_map_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    let texture_handle: Handle<Texture> = asset_server.load("textures/spider_sprite.png");
    let enemy_sprite_material = materials.add(texture_handle.into());
    commands
        .spawn_bundle(SpriteBundle {
            material: enemy_sprite_material,
            transform: Transform::from_xyz(position.x, position.y, 12.0),
            ..Default::default()
        })
        .insert(GameState::MapView)
        .insert(Enemy::default());
}