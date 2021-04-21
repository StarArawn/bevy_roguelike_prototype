use bevy::{prelude::*, render::camera::RenderLayers};

pub mod spawner;

#[derive(Default)]
pub struct Enemy {}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    _texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    position: Vec2,
) {
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(position.x, position.y, 12.0))
        .insert(Enemy::default())
        .with_children(|child_builder| {
            let texture_handle: Handle<Texture> = asset_server.load("textures/spider_sprite.png");
            let enemy_sprite_material = materials.add(texture_handle.into());
            child_builder    
                .spawn_bundle(SpriteBundle {
                    material: enemy_sprite_material,
                    ..Default::default()
                })
                .insert(RenderLayers::layer(0))
                .id();
        })
        .id();
}