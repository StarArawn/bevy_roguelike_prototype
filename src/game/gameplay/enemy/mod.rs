use bevy::{prelude::*, render::camera::RenderLayers};

use super::attributes::*;

pub mod spawner;

#[derive(Default)]
pub struct Enemy {}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    _materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    let enemy = commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(position.x, position.y, 11.0))
        .insert(Enemy::default())
        .with_children(|child_builder| {
            let texture_handle: Handle<Texture> = asset_server.load("textures/enemies/skeleton/idle.png");
            let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150.0, 150.0), 4, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            child_builder    
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.3)),
                    ..Default::default()
                })
                .insert(Timer::from_seconds(0.1, true))
                .insert(RenderLayers::layer(0))
                .id();
        })
        .id();
        
    build_basic_character_attributes(commands, enemy);
}

pub fn create_battle_enemy(
    _enemy_entity: Entity,
    child_builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Texture> = asset_server.load("textures/enemies/skeleton/idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150.0, 150.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Battle screen
    let mut transform = Transform::from_scale(Vec3::splat(5.0));
    transform.translation.x = 0.0;
    transform.translation.z = 10.0;
    child_builder    
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                flip_x: true,
                ..TextureAtlasSprite::default()
            },
            transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(RenderLayers::layer(1))
        .id();
}